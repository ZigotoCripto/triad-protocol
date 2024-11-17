import { AnchorProvider, BN, Program, Wallet } from '@coral-xyz/anchor'
import { Connection, Keypair, PublicKey } from '@solana/web3.js'
import { TriadProtocol } from './types/triad_protocol'
import IDL from './types/idl_triad_protocol.json'
import Trade from './trade'
import { formatUser } from './utils/helpers'
import { getAssociatedTokenAddress } from '@solana/spl-token'
import {
  getTokenVaultAddressSync,
  getUserPDA,
  getUserPositionPDA,
  getVaultAddressSync
} from './utils/pda'
import Stake from './stake'
import { CreateUserArgs, MintTicketArgs, RpcOptions } from './types'
import sendTransactionWithOptions from './utils/sendTransactionWithOptions'
import { bs58 } from '@coral-xyz/anchor/dist/cjs/utils/bytes'
import sendVersionedTransaction from './utils/sendVersionedTransaction'
import { TICKET_CORE_COLLECTION, TRD_MINT, VERIFIER } from './utils/constants'
import { convertSecretKeyToKeypair } from './utils/convertSecretKeyToKeypair'

export default class TriadProtocolClient {
  program: Program<TriadProtocol>
  provider: AnchorProvider
  trade: Trade
  stake: Stake

  constructor(connection: Connection, wallet: Wallet) {
    this.provider = new AnchorProvider(connection, wallet, {
      commitment: 'confirmed'
    })
    this.program = new Program(IDL as TriadProtocol, this.provider)

    this.trade = new Trade(this.program, this.provider)
    this.stake = new Stake(this.program, this.provider)
  }

  /**
   * Get User by wallet
   * @param wallet - User wallet
   *
   */
  async getUser(wallet: PublicKey) {
    const userPDA = getUserPDA(this.program.programId, wallet)

    return formatUser(await this.program.account.user.fetch(userPDA))
  }

  /**
   * Get User by wallet
   * @param wallet - User wallet
   *
   */
  async getUsers() {
    const response = await this.program.account.user.all()

    return response
      .map((item) => formatUser(item.account))
      .sort((a, b) => b.referred - a.referred)
  }

  /**
   * Check if user exists
   * @param username - User name
   *
   */
  async hasUser(wallet: PublicKey) {
    try {
      await this.program.account.user.fetch(
        getUserPDA(this.program.programId, wallet)
      )

      return true
    } catch {
      return false
    }
  }

  /**
   * Get Refferal
   * @param name - User name
   *
   */
  async getReferral(name: string) {
    try {
      const users = await this.program.account.user.all([
        {
          memcmp: {
            offset: 89 + 4,
            bytes: bs58.encode(Buffer.from(name))
          }
        }
      ])

      if (users.length > 0) {
        return users[0].publicKey.toString()
      }

      return ''
    } catch (error) {
      console.error('Error fetching referral:', error)
      return ''
    }
  }

  /**
   *  Create User
   *  @param wallet - User wallet
   *  @param name - user name
   *  @param referral - user referral
   *
   *  @param options - RPC options
   *
   */
  async createUser(
    { wallet, name, referral }: CreateUserArgs,
    options?: RpcOptions
  ) {
    return sendTransactionWithOptions(
      this.program.methods
        .createUser({
          name
        })
        .accounts({
          signer: wallet,
          referral
        }),
      options
    )
  }

  /**
   * Get User Positions with amount
   * @param wallet - User wallet
   *
   */
  async getUserPositionsWithAmount(wallet: PublicKey) {
    const userPositions: PublicKey[] = []

    const userPositionPDA = getUserPositionPDA(
      this.program.programId,
      wallet,
      new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v')
    )

    userPositions.push(userPositionPDA)

    const userPositionsWithAmount =
      await this.program.account.userPosition.fetchMultiple(userPositions)

    return userPositionsWithAmount.filter(
      (item) =>
        item &&
        parseFloat(item.totalDeposited.toString()) >
          parseFloat(item.totalWithdrawn.toString())
    )
  }

  async withdrawV1(
    {
      wallet,
      ticker,
      positionIndex
    }: {
      wallet: PublicKey
      ticker: PublicKey
      positionIndex: number
    },
    options?: RpcOptions
  ) {
    const vaultPDA = getVaultAddressSync(this.program.programId, ticker)

    const userPositionPDA = getUserPositionPDA(
      this.program.programId,
      wallet,
      ticker
    )

    const VaultTokenAccountPDA = getTokenVaultAddressSync(
      this.program.programId,
      vaultPDA
    )
    const userTokenAccount = await getAssociatedTokenAddress(
      new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v'),
      this.provider.wallet.publicKey
    )

    return sendTransactionWithOptions(
      this.program.methods.withdrawV1(positionIndex).accounts({
        signer: wallet,
        userPosition: userPositionPDA,
        userTokenAccount,
        vault: vaultPDA,
        vaultTokenAccount: VaultTokenAccountPDA
      }),
      options
    )
  }

  async createCollection(
    args: {
      collectionName: string
      collectionSymbol: string
      supply: number
    },
    options?: RpcOptions
  ) {
    const keyPair = Keypair.generate()

    const ix = await this.program.methods
      .createCollection({
        supply: new BN(args.supply),
        name: args.collectionName,
        symbol: args.collectionSymbol
      })
      .accounts({
        signer: this.provider.wallet.publicKey,
        coreCollection: keyPair.publicKey
      })
      .instruction()

    return sendVersionedTransaction(this.provider, [ix], options, keyPair)
  }

  async mintTicket(
    {
      collectionSymbol,
      number,
      discount,
      isBoosted,
      rarity,
      verifier,
      nftMint
    }: MintTicketArgs,
    options?: RpcOptions
  ) {
    const asset = Keypair.generate()

    const userNftAta = await getAssociatedTokenAddress(
      nftMint,
      this.provider.wallet.publicKey
    )

    const ix = await this.program.methods
      .mintTicket({
        number: new BN(number),
        collectionSymbol,
        discount: new BN(discount),
        isBoosted,
        rarity
      })
      .accounts({
        signer: this.provider.wallet.publicKey,
        asset: asset.publicKey,
        nftMint,
        userNftAta,
        trdMint: TRD_MINT,
        verifier: VERIFIER,
        coreCollection: TICKET_CORE_COLLECTION
      })
      .instruction()

    return sendVersionedTransaction(
      this.provider,
      [ix],
      options,
      asset,
      [],
      convertSecretKeyToKeypair(verifier)
    )
  }
}
