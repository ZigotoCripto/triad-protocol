import { Stake, StakeVault } from './../types/stake'
import { User } from './../types'
import { Market, WinningDirection } from '../types/trade'
import { PublicKey } from '@solana/web3.js'

export const encodeString = (value: string, alloc = 32): number[] => {
  const buffer = Buffer.alloc(alloc, 32)

  buffer.write(value)

  return Array(...buffer)
}

export const decodeString = (bytes: number[]): string => {
  const buffer = Buffer.from(bytes)
  return buffer.toString('utf8').trim()
}

export const formatStakeVault = (stakeVault: any): StakeVault => {
  return {
    name: stakeVault.name,
    collection: stakeVault.collection,
    slots: stakeVault.slots.toNumber(),
    amount: stakeVault.amount.toNumber(),
    isLocked: stakeVault.isLocked,
    tokenMint: stakeVault.tokenMint.toBase58(),
    tokenDecimals: stakeVault.tokenDecimals,
    amountPaid: stakeVault.amountPaid.toNumber(),
    nftStaked: stakeVault.nftStaked.toNumber(),
    week: stakeVault.week,
    initTs: stakeVault.initTs.toNumber(),
    endTs: stakeVault.endTs.toNumber(),
    authority: stakeVault.authority.toBase58(),
    tokenStaked:
      stakeVault.tokenStaked.toNumber() / 10 ** stakeVault.tokenDecimals
  }
}

export const formatStake = (stake: any): Stake => {
  return {
    name: stake.name,
    stakeVault: stake.stakeVault.toBase58(),
    authority: stake.authority.toBase58(),
    initTs: stake.initTs.toNumber(),
    withdrawTs: stake.withdrawTs.toNumber(),
    mint: stake.mint.toBase58(),
    claimedTs: stake?.claimedTs?.toNumber(),
    boost: stake?.boost,
    claimed: (stake?.claimed?.toNumber() || 0) / 10 ** 6,
    available: (stake?.available?.toNumber() || 0) / 10 ** 6,
    amount: stake?.amount?.toNumber() || 1
  }
}

export const formatUser = (user: any): User => {
  return {
    ts: user.ts.toNumber(),
    authority: user.authority.toBase58(),
    referral: user.referral,
    referred: user.referred.toNumber(),
    swapsMade: user.swapsMade,
    swaps: user.swaps,
    staked: user.staked.toNumber(),
    name: user.name
  }
}

export const formatMarket = (account: any, address: PublicKey): Market => {
  return {
    bump: account.bump,
    address: address.toString(),
    authority: account.authority.toString(),
    marketId: account.marketId.toString(),
    hypePrice: account.hypePrice.toString(),
    flopPrice: account.flopPrice.toString(),
    hypeLiquidity: account.hypeLiquidity.toString(),
    flopLiquidity: account.flopLiquidity.toString(),
    hypeShares: account.hypeShares.toString(),
    flopShares: account.flopShares.toString(),
    volume: account.volume.toString(),
    mint: account.mint.toString(),
    updateTs: account.updateTs.toString(),
    openedOrders: account.openedOrders.toString(),
    nextOrderId: account.nextOrderId.toString(),
    feeBps: account.feeBps,
    isActive: account.isActive,
    marketStart: account.marketStart.toString(),
    marketEnd: account.marketEnd.toString(),
    question: Buffer.from(account.question).toString().replace(/\0+$/, ''),
    nftHoldersFeeAvailable: account.nftHoldersFeeAvailable.toString(),
    nftHoldersFeeClaimed: account.nftHoldersFeeClaimed.toString(),
    marketFeeAvailable: account.marketFeeAvailable.toString(),
    marketFeeClaimed: account.marketFeeClaimed.toString(),
    winningDirection:
      WinningDirection[Object.keys(account.winningDirection)[0].toUpperCase()],
    marketLiquidityAtStart:
      account.marketLiquidityAtStart.toString() > 0
        ? account.marketLiquidityAtStart.toString()
        : '500000000'
  }
}

export const accountToMarketV1 = (account: any, address: PublicKey): Market => {
  return {
    bump: account.bump,
    address: address.toString(),
    authority: account.authority.toString(),
    marketId: account.marketId.toString(),
    hypePrice: account.hypePrice.toString(),
    flopPrice: account.flopPrice.toString(),
    hypeLiquidity: account.hypeLiquidity.toString(),
    flopLiquidity: account.flopLiquidity.toString(),
    hypeShares: account.totalHypeShares.toString(),
    flopShares: account.totalFlopShares.toString(),
    volume: account.totalVolume.toString(),
    mint: account.mint.toString(),
    updateTs: account.updateTs.toString(),
    openedOrders: account.openOrdersCount.toString(),
    nextOrderId: account.nextOrderId.toString(),
    feeBps: account.feeBps,
    isActive: account.isActive,
    marketStart: account.currentQuestionStart.toString(),
    marketEnd: account.currentQuestionEnd.toString(),
    question: Buffer.from(account.currentQuestion)
      .toString()
      .replace(/\0+$/, ''),
    nftHoldersFeeAvailable: '0',
    nftHoldersFeeClaimed: '0',
    marketFeeAvailable: '0',
    marketFeeClaimed: '0',
    winningDirection:
      WinningDirection[
        Object.keys(
          account.previousResolvedQuestion.winningDirection
        )[0].toUpperCase()
      ],
    marketLiquidityAtStart: '0'
  }
}
