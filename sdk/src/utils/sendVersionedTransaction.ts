import {
  TransactionInstruction,
  TransactionMessage,
  VersionedTransaction,
  ComputeBudgetProgram
} from '@solana/web3.js'
import { RpcOptions } from '../types'
import { AddressLookupTableAccount } from '@solana/web3.js'
import { AnchorProvider } from '@coral-xyz/anchor'
import { Keypair } from '@solana/web3.js'

const sendVersionedTransaction = async (
  provider: AnchorProvider,
  ixs: TransactionInstruction[],
  options?: RpcOptions,
  payer?: Keypair,
  addressLookupTableAccounts?: AddressLookupTableAccount[],
  verifier?: Keypair
): Promise<string> => {
  if (options?.microLamports) {
    ixs.push(
      ComputeBudgetProgram.setComputeUnitPrice({
        microLamports: options.microLamports
      })
    )
  }

  const { blockhash } = await provider.connection.getLatestBlockhash()

  const tx = new VersionedTransaction(
    new TransactionMessage({
      instructions: ixs,
      recentBlockhash: blockhash,
      payerKey: provider.publicKey
    }).compileToV0Message(addressLookupTableAccounts)
  )

  let signers = []

  if (payer) {
    tx.sign([payer])
    signers.push(payer)
  }

  if (verifier) {
    tx.sign([verifier])
    signers.push(verifier)
  }

  return provider.sendAndConfirm(tx, signers, {
    skipPreflight: options?.skipPreflight,
    commitment: 'confirmed'
  })
}

export default sendVersionedTransaction
