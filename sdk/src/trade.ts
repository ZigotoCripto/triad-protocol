import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { TriadProtocol } from './types/triad_protocol'
import {
  AddressLookupTableAccount,
  ComputeBudgetProgram,
  PublicKey,
  SystemProgram,
  TransactionInstruction
} from '@solana/web3.js'
import {
  InitializeMarketArgs,
  Market,
  OpenOrderArgs,
  OrderDirection
} from './types/trade'
import { RpcOptions } from './types'
import BN from 'bn.js'
import { SOL_MINT, TRD_DECIMALS, TRD_MINT } from './utils/constants'
import { accountToMarketV1, encodeString, formatMarket } from './utils/helpers'
import { getMarketPDA, getUserTradePDA } from './utils/pda/trade'
import { getTokenATA, getUserPDA } from './utils/pda'
import sendVersionedTransaction from './utils/sendVersionedTransaction'
import sendTransactionWithOptions from './utils/sendTransactionWithOptions'
import { swap } from './utils/swap'
import {
  createTransferCheckedInstruction,
  TOKEN_2022_PROGRAM_ID
} from '@solana/spl-token'
import { jupSwap } from './utils/jup-swap'

export default class Trade {
  program: Program<TriadProtocol>
  provider: AnchorProvider
  mint: PublicKey = TRD_MINT

  constructor(program: Program<TriadProtocol>, provider: AnchorProvider) {
    this.provider = provider
    this.program = program
  }

  /**
   * Get all Markets
   */
  async getAllMarkets(): Promise<Market[]> {
    const marketV1 = await this.program.account.market
      .all()
      .then((markets) =>
        markets.map(({ account, publicKey }) =>
          accountToMarketV1(account, publicKey)
        )
      )

    const marketV2 = await this.program.account.marketV2
      .all()
      .then((markets) =>
        markets.map(({ account, publicKey }) =>
          formatMarket(account, publicKey)
        )
      )

    return [...marketV1, ...marketV2]
  }

  /**
   * Get Market by ID
   * @param marketId - The ID of the market
   *
   */
  async getMarketById(marketId: number): Promise<Market> {
    const marketPDA = getMarketPDA(this.program.programId, marketId)

    const response = await this.program.account.marketV2.fetch(marketPDA)

    return formatMarket(response, marketPDA)
  }

  /**
   * Get Market by Address
   * @param address - The address of the market
   *
   */
  async getMarketByAddress(address: PublicKey): Promise<Market> {
    const account = await this.program.account.marketV2.fetch(address)

    return formatMarket(account, address)
  }

  /**
   * Get User Trade
   * @param user - The user's public key
   *
   */
  async getUserTrade(user: PublicKey) {
    const userTradePDA = getUserTradePDA(this.program.programId, user)

    return this.program.account.userTrade.fetch(userTradePDA)
  }

  /**
   * Initialize Market
   * @param market id - new markert id - length + 1
   * @param name - PYTH/TRD JUP/TRD DRIFT/TRD
   *
   * @param options - RPC options
   *
   */
  async initializeMarket(
    { marketId, name, startTime, endTime, question }: InitializeMarketArgs,
    options?: RpcOptions
  ) {
    return sendTransactionWithOptions(
      this.program.methods
        .initializeMarket({
          marketId: new BN(marketId),
          name: name,
          question: encodeString(question, 80),
          startTime: new BN(startTime),
          endTime: new BN(endTime)
        })
        .accounts({
          signer: this.provider.publicKey,
          mint: this.mint
        }),
      options
    )
  }

  /**
   * Open Order
   * @param marketId - The ID of the market
   * @param amount - The amount of the order
   * @param direction - The direction of the order
   * @param token - The token to use for the order
   * @param comment - The comment of the order
   *
   * @param options - RPC options
   *
   */
  async openOrder(
    { marketId, amount, direction, token, comment }: OpenOrderArgs,
    options?: RpcOptions
  ): Promise<string> {
    const marketPDA = getMarketPDA(this.program.programId, marketId)
    const userTradePDA = getUserTradePDA(
      this.program.programId,
      this.provider.publicKey
    )
    const userPDA = getUserPDA(this.program.programId, this.provider.publicKey)

    const ixs: TransactionInstruction[] = []
    const addressLookupTableAccounts: AddressLookupTableAccount[] = []

    let amountInTRD = amount * 10 ** TRD_DECIMALS

    try {
      await this.program.account.userTrade.fetch(userTradePDA)
    } catch {
      ixs.push(
        await this.program.methods
          .createUserTrade()
          .accounts({
            signer: this.provider.publicKey,
            user: userPDA
          })
          .instruction()
      )
    }

    if (token !== TRD_MINT.toBase58()) {
      const {
        setupInstructions,
        swapIxs,
        addressLookupTableAccounts,
        trdAmount
      } = await swap({
        connection: this.provider.connection,
        wallet: this.provider.publicKey.toBase58(),
        inToken: token,
        amount
      })

      amountInTRD = trdAmount

      if (swapIxs.length === 0) {
        return
      }

      ixs.push(...setupInstructions)
      ixs.push(...swapIxs)
      addressLookupTableAccounts.push(...addressLookupTableAccounts)
    }

    ixs.push(
      await this.program.methods
        .openOrder({
          amount: new BN(amountInTRD),
          direction: direction,
          comment: encodeString(comment, 64)
        })
        .accounts({
          signer: this.provider.publicKey,
          market: marketPDA,
          userTrade: userTradePDA,
          mint: this.mint
        })
        .instruction()
    )

    return sendVersionedTransaction(
      this.provider,
      ixs,
      options,
      undefined,
      addressLookupTableAccounts
    )
  }

  /**
   * Close Order
   * @param marketId - The ID of the market
   * @param orderId - The ID of the order
   *
   * @param options - RPC options
   *
   */
  async closeOrder(
    { marketId, orderId }: { marketId: number; orderId: number },
    options?: RpcOptions
  ): Promise<string> {
    const marketPDA = getMarketPDA(this.program.programId, marketId)
    const userTradePDA = getUserTradePDA(
      this.program.programId,
      this.provider.publicKey
    )

    return sendTransactionWithOptions(
      this.program.methods.closeOrder(new BN(orderId)).accounts({
        signer: this.provider.publicKey,
        market: marketPDA,
        mint: this.mint,
        userTrade: userTradePDA
      }),
      options
    )
  }

  /**
   * Resolve Market
   * @param marketId - The ID of the market
   * @param winningDirection - The winning direction of the market
   *
   * @param options - RPC options
   *
   */
  async resolveMarket(
    {
      marketId,
      winningDirection
    }: {
      marketId: number
      winningDirection:
        | {
            hype: {}
          }
        | {
            flop: {}
          }
        | {
            none: {}
          }
    },
    options?: RpcOptions
  ): Promise<string> {
    const marketPDA = getMarketPDA(this.program.programId, marketId)

    const method = this.program.methods
      .resolveMarket(winningDirection)
      .accounts({
        signer: this.provider.publicKey,
        market: marketPDA
      })

    return sendTransactionWithOptions(method, options)
  }

  /**
   * Settle an order
   * @param marketId - The ID of the market
   * @param orderId - The ID of the order to settle
   *
   * @param options - RPC options
   *
   */
  async settleOrder(
    { marketId, orderId }: { marketId: number; orderId: number },
    options?: RpcOptions
  ): Promise<string> {
    const marketPDA = getMarketPDA(this.program.programId, marketId)
    const userTradePDA = getUserTradePDA(
      this.program.programId,
      this.provider.publicKey
    )

    return sendTransactionWithOptions(
      this.program.methods.settleOrder(new BN(orderId)).accounts({
        signer: this.provider.publicKey,
        userTrade: userTradePDA,
        market: marketPDA,
        mint: this.mint
      }),
      options
    )
  }

  /**
   * Add Liquidity
   * @param marketId - The ID of the market
   * @param amount - The amount of the order
   * @param direction - The direction of the order
   *
   * @param options - RPC options
   *
   */
  async addLiquidity(
    {
      marketId,
      amount,
      direction
    }: { marketId: number; amount: number; direction: OrderDirection },
    options?: RpcOptions
  ): Promise<string> {
    const marketPDA = getMarketPDA(this.program.programId, marketId)

    return sendTransactionWithOptions(
      this.program.methods
        .addLiquidity({
          amount: new BN(amount * 10 ** TRD_DECIMALS),
          direction: direction
        })
        .accounts({
          signer: this.provider.publicKey,
          market: marketPDA,
          mint: this.mint
        }),
      options
    )
  }

  /**
   * Collect Fee
   * @param marketId - The ID of the market
   *
   * @param options - RPC options
   *
   */
  async collectFee(
    { marketId, vault }: { marketId: number; vault: PublicKey },
    options?: RpcOptions
  ) {
    const marketPDA = getMarketPDA(this.program.programId, marketId)

    const ixs: TransactionInstruction[] = []

    ixs.push(
      await this.program.methods
        .collectFee()
        .accounts({
          signer: this.provider.publicKey,
          market: marketPDA,
          mint: this.mint
        })
        .instruction()
    )

    const market = await this.getMarketById(marketId)

    const marketFee =
      parseFloat(market.marketFeeAvailable) -
      parseFloat(market.marketFeeClaimed)
    const nftFee =
      parseFloat(market.nftHoldersFeeAvailable) -
      parseFloat(market.nftHoldersFeeClaimed)

    const totalFee = marketFee + nftFee

    if (totalFee / 10 ** TRD_DECIMALS < 100) {
      return
    }

    const {
      setupInstructions,
      swapIxs,
      addressLookupTableAccounts,
      cleanupInstruction,
      otherAmountThreshold
    } = await jupSwap({
      connection: this.provider.connection,
      wallet: this.provider.publicKey.toBase58(),
      inToken: TRD_MINT.toBase58(),
      outToken: SOL_MINT.toBase58(),
      amount: parseInt(totalFee.toFixed())
    })

    ixs.push(...setupInstructions)
    ixs.push(
      ComputeBudgetProgram.setComputeUnitLimit({
        units: 500000
      })
    )
    ixs.push(...swapIxs)
    ixs.push(cleanupInstruction)

    ixs.push(
      SystemProgram.transfer({
        fromPubkey: this.provider.publicKey,
        toPubkey: vault,
        lamports: otherAmountThreshold
      })
    )

    await sendVersionedTransaction(
      this.provider,
      ixs,
      options,
      undefined,
      addressLookupTableAccounts
    )

    return {
      feeToSwap: totalFee,
      lamport: otherAmountThreshold as number
    }
  }
}
