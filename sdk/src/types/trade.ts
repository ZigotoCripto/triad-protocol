import { PublicKey } from '@solana/web3.js'

export type Market = {
  address: string
  bump: number
  authority: string
  marketId: string
  name: string
  hypePrice: string
  flopPrice: string
  hypeLiquidity: string
  flopLiquidity: string
  hypeShares: string
  flopShares: string
  volume: string
  mint: string
  updateTs: string
  openedOrders: string
  nextOrderId: string
  feeBps: number
  feeVault: string
  isActive: boolean
  marketPrice: string
  nftHoldersFeeAvailable: string
  nftHoldersFeeClaimed: string
  marketFeeAvailable: string
  marketFeeClaimed: string
  marketStart: string
  marketEnd: string
  question: string
  winningDirection: WinningDirection
}

export enum WinningDirection {
  HYPE = 'Hype',
  FLOP = 'Flop',
  NONE = 'None'
}

export type OrderDirection = { hype: {} } | { flop: {} }

export type OrderStatus =
  | { init: {} }
  | { open: {} }
  | { closed: {} }
  | { claimed: {} }
  | { liquidated: {} }

export type OrderType = { limit: {} } | { market: {} }

export type OpenOrderArgs = {
  marketId: number
  amount: number
  direction: OrderDirection
  token: string
  comment?: string
}

export type InitializeMarketArgs = {
  marketId: number
  name: string
  startTime: number
  endTime: number
  question: string
}
