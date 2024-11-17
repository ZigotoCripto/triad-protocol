import axios from 'axios'
import {
  AddressLookupTableAccount,
  Connection,
  PublicKey,
  TransactionInstruction
} from '@solana/web3.js'

export const jupSwap = async ({
  connection,
  wallet,
  inToken,
  outToken,
  amount
}: {
  connection: Connection
  wallet: string
  inToken: string
  outToken: string
  amount: number
}) => {
  try {
    const quoteResponse = await axios.get(
      `https://quote-api.jup.ag/v6/quote?inputMint=${inToken}&outputMint=${outToken}&amount=${amount}&slippageBps=100`
    )

    const { data: quoteData } = quoteResponse

    const swapResponse = await axios.post(
      'https://quote-api.jup.ag/v6/swap-instructions',
      {
        userPublicKey: wallet,
        quoteResponse: quoteData
      }
    )

    const {
      setupInstructions,
      swapInstruction,
      addressLookupTableAddresses,
      cleanupInstruction
    } = swapResponse.data

    return {
      swapIxs: [deserializeInstruction(swapInstruction)],
      addressLookupTableAccounts: await getAddressLookupTableAccounts(
        connection,
        addressLookupTableAddresses
      ),
      setupInstructions: setupInstructions.map(deserializeInstruction),
      cleanupInstruction: deserializeInstruction(cleanupInstruction),
      otherAmountThreshold: quoteData.otherAmountThreshold
    }
  } catch (e) {
    console.log(e?.response?.data)
  }
}

const deserializeInstruction = (instruction: any) => {
  return new TransactionInstruction({
    programId: new PublicKey(instruction.programId),
    keys: instruction.accounts.map((key: any) => ({
      pubkey: new PublicKey(key.pubkey),
      isSigner: key.isSigner,
      isWritable: key.isWritable
    })),
    data: Buffer.from(instruction.data, 'base64')
  })
}

export const getAddressLookupTableAccounts = async (
  connection: Connection,
  keys: string[]
): Promise<AddressLookupTableAccount[]> => {
  const addressLookupTableAccountInfos =
    await connection.getMultipleAccountsInfo(
      keys.map((key) => new PublicKey(key))
    )

  return addressLookupTableAccountInfos.reduce((acc, accountInfo, index) => {
    const addressLookupTableAddress = keys[index]
    if (accountInfo) {
      const addressLookupTableAccount = new AddressLookupTableAccount({
        key: new PublicKey(addressLookupTableAddress),
        state: AddressLookupTableAccount.deserialize(accountInfo.data)
      })
      acc.push(addressLookupTableAccount)
    }

    return acc
  }, new Array<AddressLookupTableAccount>())
}
