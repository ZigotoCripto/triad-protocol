/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/triad_protocol.json`.
 */
export type TriadProtocol = {
  address: 'TRDwq3BN4mP3m9KsuNUWSN6QDff93VKGSwE95Jbr9Ss'
  metadata: {
    name: 'triadProtocol'
    version: '0.1.4'
    spec: '0.1.0'
    description: 'Triad protocol, trade solana projects'
  }
  instructions: [
    {
      name: 'addLiquidity'
      discriminator: [181, 157, 89, 67, 143, 182, 52, 72]
      accounts: [
        {
          name: 'signer'
          writable: true
          signer: true
        },
        {
          name: 'market'
          writable: true
        },
        {
          name: 'mint'
          writable: true
        },
        {
          name: 'userFromAta'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'account'
                path: 'signer'
              },
              {
                kind: 'account'
                path: 'tokenProgram'
              },
              {
                kind: 'account'
                path: 'mint'
              }
            ]
            program: {
              kind: 'const'
              value: [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          name: 'marketToAta'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'account'
                path: 'market'
              },
              {
                kind: 'account'
                path: 'tokenProgram'
              },
              {
                kind: 'account'
                path: 'mint'
              }
            ]
            program: {
              kind: 'const'
              value: [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          name: 'tokenProgram'
          address: 'TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb'
        },
        {
          name: 'associatedTokenProgram'
          address: 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'
        },
        {
          name: 'systemProgram'
          address: '11111111111111111111111111111111'
        }
      ]
      args: [
        {
          name: 'args'
          type: {
            defined: {
              name: 'addLiquidityArgs'
            }
          }
        }
      ]
    },
    {
      name: 'claimStakeRewards'
      discriminator: [107, 91, 233, 196, 211, 47, 218, 21]
      accounts: [
        {
          name: 'signer'
          writable: true
          signer: true
        },
        {
          name: 'verifier'
          writable: true
          signer: true
        },
        {
          name: 'stakeVault'
          writable: true
        },
        {
          name: 'stake'
          writable: true
        },
        {
          name: 'mint'
          writable: true
        },
        {
          name: 'fromAta'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'account'
                path: 'stakeVault'
              },
              {
                kind: 'account'
                path: 'tokenProgram'
              },
              {
                kind: 'account'
                path: 'mint'
              }
            ]
            program: {
              kind: 'const'
              value: [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          name: 'toAta'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'account'
                path: 'signer'
              },
              {
                kind: 'account'
                path: 'tokenProgram'
              },
              {
                kind: 'account'
                path: 'mint'
              }
            ]
            program: {
              kind: 'const'
              value: [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          name: 'tokenProgram'
          address: 'TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb'
        },
        {
          name: 'associatedTokenProgram'
          address: 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'
        },
        {
          name: 'systemProgram'
          address: '11111111111111111111111111111111'
        }
      ]
      args: [
        {
          name: 'args'
          type: {
            defined: {
              name: 'claimStakeRewardsArgs'
            }
          }
        }
      ]
      returns: 'u64'
    },
    {
      name: 'closeOrder'
      discriminator: [90, 103, 209, 28, 7, 63, 168, 4]
      accounts: [
        {
          name: 'signer'
          writable: true
          signer: true
        },
        {
          name: 'userTrade'
          writable: true
        },
        {
          name: 'market'
          writable: true
        },
        {
          name: 'mint'
          writable: true
        },
        {
          name: 'userAta'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'account'
                path: 'signer'
              },
              {
                kind: 'account'
                path: 'tokenProgram'
              },
              {
                kind: 'account'
                path: 'mint'
              }
            ]
            program: {
              kind: 'const'
              value: [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          name: 'marketVault'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'account'
                path: 'market'
              },
              {
                kind: 'account'
                path: 'tokenProgram'
              },
              {
                kind: 'account'
                path: 'mint'
              }
            ]
            program: {
              kind: 'const'
              value: [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          name: 'tokenProgram'
          address: 'TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb'
        },
        {
          name: 'associatedTokenProgram'
          address: 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'
        },
        {
          name: 'systemProgram'
          address: '11111111111111111111111111111111'
        }
      ]
      args: [
        {
          name: 'orderId'
          type: 'u64'
        }
      ]
    },
    {
      name: 'collectFee'
      discriminator: [60, 173, 247, 103, 4, 93, 130, 48]
      accounts: [
        {
          name: 'signer'
          writable: true
          signer: true
        },
        {
          name: 'market'
          writable: true
        },
        {
          name: 'mint'
          writable: true
        },
        {
          name: 'marketAta'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'account'
                path: 'market'
              },
              {
                kind: 'account'
                path: 'tokenProgram'
              },
              {
                kind: 'account'
                path: 'mint'
              }
            ]
            program: {
              kind: 'const'
              value: [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          name: 'signerAta'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'account'
                path: 'signer'
              },
              {
                kind: 'account'
                path: 'tokenProgram'
              },
              {
                kind: 'account'
                path: 'mint'
              }
            ]
            program: {
              kind: 'const'
              value: [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          name: 'tokenProgram'
          address: 'TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb'
        },
        {
          name: 'associatedTokenProgram'
          address: 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'
        },
        {
          name: 'systemProgram'
          address: '11111111111111111111111111111111'
        }
      ]
      args: []
    },
    {
      name: 'createCollection'
      discriminator: [156, 251, 92, 54, 233, 2, 16, 82]
      accounts: [
        {
          name: 'signer'
          writable: true
          signer: true
        },
        {
          name: 'collection'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'const'
                value: [99, 111, 108, 108, 101, 99, 116, 105, 111, 110]
              },
              {
                kind: 'arg'
                path: 'args.symbol'
              }
            ]
          }
        },
        {
          name: 'metaplexProgram'
          address: 'CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d'
        },
        {
          name: 'tokenProgram'
          address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'
        },
        {
          name: 'associatedTokenProgram'
          address: 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'
        },
        {
          name: 'systemProgram'
          address: '11111111111111111111111111111111'
        }
      ]
      args: [
        {
          name: 'args'
          type: {
            defined: {
              name: 'createCollectionArgs'
            }
          }
        }
      ]
    },
    {
      name: 'createUser'
      discriminator: [108, 227, 130, 130, 252, 109, 75, 218]
      accounts: [
        {
          name: 'signer'
          writable: true
          signer: true
        },
        {
          name: 'referral'
          writable: true
        },
        {
          name: 'user'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'const'
                value: [117, 115, 101, 114]
              },
              {
                kind: 'account'
                path: 'signer'
              }
            ]
          }
        },
        {
          name: 'systemProgram'
          address: '11111111111111111111111111111111'
        }
      ]
      args: [
        {
          name: 'args'
          type: {
            defined: {
              name: 'createUserArgs'
            }
          }
        }
      ]
    },
    {
      name: 'createUserTrade'
      discriminator: [232, 235, 58, 194, 135, 248, 153, 1]
      accounts: [
        {
          name: 'signer'
          writable: true
          signer: true
        },
        {
          name: 'user'
          writable: true
        },
        {
          name: 'userTrade'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'const'
                value: [117, 115, 101, 114, 95, 116, 114, 97, 100, 101]
              },
              {
                kind: 'account'
                path: 'signer'
              }
            ]
          }
        },
        {
          name: 'systemProgram'
          address: '11111111111111111111111111111111'
        }
      ]
      args: []
    },
    {
      name: 'initializeMarket'
      discriminator: [35, 35, 189, 193, 155, 48, 170, 203]
      accounts: [
        {
          name: 'signer'
          writable: true
          signer: true
        },
        {
          name: 'market'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'const'
                value: [109, 97, 114, 107, 101, 116]
              },
              {
                kind: 'arg'
                path: 'args.market_id'
              }
            ]
          }
        },
        {
          name: 'mint'
          writable: true
        },
        {
          name: 'marketAta'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'account'
                path: 'market'
              },
              {
                kind: 'account'
                path: 'tokenProgram'
              },
              {
                kind: 'account'
                path: 'mint'
              }
            ]
            program: {
              kind: 'const'
              value: [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          name: 'tokenProgram'
          address: 'TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb'
        },
        {
          name: 'associatedTokenProgram'
          address: 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'
        },
        {
          name: 'systemProgram'
          address: '11111111111111111111111111111111'
        }
      ]
      args: [
        {
          name: 'args'
          type: {
            defined: {
              name: 'initializeMarketArgs'
            }
          }
        }
      ]
    },
    {
      name: 'openOrder'
      discriminator: [206, 88, 88, 143, 38, 136, 50, 224]
      accounts: [
        {
          name: 'signer'
          writable: true
          signer: true
        },
        {
          name: 'userTrade'
          writable: true
        },
        {
          name: 'market'
          writable: true
        },
        {
          name: 'mint'
          writable: true
        },
        {
          name: 'userFromAta'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'account'
                path: 'signer'
              },
              {
                kind: 'account'
                path: 'tokenProgram'
              },
              {
                kind: 'account'
                path: 'mint'
              }
            ]
            program: {
              kind: 'const'
              value: [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          name: 'marketToAta'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'account'
                path: 'market'
              },
              {
                kind: 'account'
                path: 'tokenProgram'
              },
              {
                kind: 'account'
                path: 'mint'
              }
            ]
            program: {
              kind: 'const'
              value: [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          name: 'tokenProgram'
          address: 'TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb'
        },
        {
          name: 'associatedTokenProgram'
          address: 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'
        },
        {
          name: 'systemProgram'
          address: '11111111111111111111111111111111'
        }
      ]
      args: [
        {
          name: 'args'
          type: {
            defined: {
              name: 'openOrderArgs'
            }
          }
        }
      ]
    },
    {
      name: 'payoutOrder'
      discriminator: [247, 233, 158, 228, 63, 32, 236, 113]
      accounts: [
        {
          name: 'signer'
          writable: true
          signer: true
        },
        {
          name: 'userTrade'
          writable: true
        },
        {
          name: 'market'
          writable: true
        },
        {
          name: 'mint'
          writable: true
        },
        {
          name: 'userAta'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'account'
                path: 'signer'
              },
              {
                kind: 'account'
                path: 'tokenProgram'
              },
              {
                kind: 'account'
                path: 'mint'
              }
            ]
            program: {
              kind: 'const'
              value: [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          name: 'marketVault'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'account'
                path: 'market'
              },
              {
                kind: 'account'
                path: 'tokenProgram'
              },
              {
                kind: 'account'
                path: 'mint'
              }
            ]
            program: {
              kind: 'const'
              value: [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          name: 'tokenProgram'
          address: 'TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb'
        },
        {
          name: 'associatedTokenProgram'
          address: 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'
        },
        {
          name: 'systemProgram'
          address: '11111111111111111111111111111111'
        }
      ]
      args: [
        {
          name: 'orderId'
          type: 'u64'
        }
      ]
    },
    {
      name: 'requestWithdrawStake'
      discriminator: [175, 9, 77, 31, 145, 136, 30, 207]
      accounts: [
        {
          name: 'signer'
          writable: true
          signer: true
        },
        {
          name: 'stakeVault'
          writable: true
        },
        {
          name: 'user'
          writable: true
        },
        {
          name: 'stake'
          writable: true
        },
        {
          name: 'mint'
          writable: true
        },
        {
          name: 'tokenProgram'
          address: 'TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb'
        },
        {
          name: 'systemProgram'
          address: '11111111111111111111111111111111'
        }
      ]
      args: []
    },
    {
      name: 'resolveMarket'
      discriminator: [155, 23, 80, 173, 46, 74, 23, 239]
      accounts: [
        {
          name: 'signer'
          writable: true
          signer: true
        },
        {
          name: 'market'
          writable: true
        },
        {
          name: 'systemProgram'
          address: '11111111111111111111111111111111'
        }
      ]
      args: [
        {
          name: 'winningDirection'
          type: {
            defined: {
              name: 'winningDirection'
            }
          }
        }
      ]
    },
    {
      name: 'resolveMarketV1'
      discriminator: [67, 202, 40, 49, 111, 136, 234, 183]
      accounts: [
        {
          name: 'signer'
          writable: true
          signer: true
        },
        {
          name: 'market'
          writable: true
        },
        {
          name: 'systemProgram'
          address: '11111111111111111111111111111111'
        }
      ]
      args: [
        {
          name: 'winningDirection'
          type: {
            defined: {
              name: 'winningDirection'
            }
          }
        }
      ]
    },
    {
      name: 'settleOrder'
      discriminator: [80, 74, 204, 34, 12, 183, 66, 66]
      accounts: [
        {
          name: 'signer'
          writable: true
          signer: true
        },
        {
          name: 'userTrade'
          writable: true
        },
        {
          name: 'market'
          writable: true
        },
        {
          name: 'mint'
          writable: true
        },
        {
          name: 'userAta'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'account'
                path: 'signer'
              },
              {
                kind: 'account'
                path: 'tokenProgram'
              },
              {
                kind: 'account'
                path: 'mint'
              }
            ]
            program: {
              kind: 'const'
              value: [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          name: 'marketVault'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'account'
                path: 'market'
              },
              {
                kind: 'account'
                path: 'tokenProgram'
              },
              {
                kind: 'account'
                path: 'mint'
              }
            ]
            program: {
              kind: 'const'
              value: [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          name: 'tokenProgram'
          address: 'TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb'
        },
        {
          name: 'associatedTokenProgram'
          address: 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'
        },
        {
          name: 'systemProgram'
          address: '11111111111111111111111111111111'
        }
      ]
      args: [
        {
          name: 'orderId'
          type: 'u64'
        }
      ]
    },
    {
      name: 'stakeToken'
      discriminator: [191, 127, 193, 101, 37, 96, 87, 211]
      accounts: [
        {
          name: 'signer'
          writable: true
          signer: true
        },
        {
          name: 'stakeVault'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'const'
                value: [115, 116, 97, 107, 101, 95, 118, 97, 117, 108, 116]
              },
              {
                kind: 'arg'
                path: 'args.stake_vault'
              }
            ]
          }
        },
        {
          name: 'user'
          writable: true
        },
        {
          name: 'stake'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'const'
                value: [115, 116, 97, 107, 101]
              },
              {
                kind: 'account'
                path: 'signer'
              },
              {
                kind: 'arg'
                path: 'args.name'
              }
            ]
          }
        },
        {
          name: 'mint'
          writable: true
        },
        {
          name: 'fromAta'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'account'
                path: 'signer'
              },
              {
                kind: 'account'
                path: 'tokenProgram'
              },
              {
                kind: 'account'
                path: 'mint'
              }
            ]
            program: {
              kind: 'const'
              value: [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          name: 'toAta'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'account'
                path: 'stakeVault'
              },
              {
                kind: 'account'
                path: 'tokenProgram'
              },
              {
                kind: 'account'
                path: 'mint'
              }
            ]
            program: {
              kind: 'const'
              value: [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          name: 'tokenProgram'
          address: 'TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb'
        },
        {
          name: 'associatedTokenProgram'
          address: 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'
        },
        {
          name: 'systemProgram'
          address: '11111111111111111111111111111111'
        }
      ]
      args: [
        {
          name: 'args'
          type: {
            defined: {
              name: 'stakeTokenArgs'
            }
          }
        }
      ]
    },
    {
      name: 'updateStakeBoost'
      discriminator: [239, 85, 19, 140, 235, 236, 88, 70]
      accounts: [
        {
          name: 'signer'
          writable: true
          signer: true
        },
        {
          name: 'stake'
          writable: true
        },
        {
          name: 'systemProgram'
          address: '11111111111111111111111111111111'
        }
      ]
      args: []
    },
    {
      name: 'updateStakeVault'
      discriminator: [84, 171, 100, 153, 126, 215, 229, 68]
      accounts: [
        {
          name: 'signer'
          writable: true
          signer: true
        },
        {
          name: 'stakeVault'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'const'
                value: [115, 116, 97, 107, 101, 95, 118, 97, 117, 108, 116]
              },
              {
                kind: 'arg'
                path: 'args.stake_vault'
              }
            ]
          }
        },
        {
          name: 'mint'
          writable: true
        },
        {
          name: 'fromAta'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'account'
                path: 'signer'
              },
              {
                kind: 'account'
                path: 'tokenProgram'
              },
              {
                kind: 'account'
                path: 'mint'
              }
            ]
            program: {
              kind: 'const'
              value: [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          name: 'toAta'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'account'
                path: 'stakeVault'
              },
              {
                kind: 'account'
                path: 'tokenProgram'
              },
              {
                kind: 'account'
                path: 'mint'
              }
            ]
            program: {
              kind: 'const'
              value: [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          name: 'tokenProgram'
          address: 'TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb'
        },
        {
          name: 'associatedTokenProgram'
          address: 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'
        },
        {
          name: 'systemProgram'
          address: '11111111111111111111111111111111'
        }
      ]
      args: [
        {
          name: 'args'
          type: {
            defined: {
              name: 'updateStakeVaultArgs'
            }
          }
        }
      ]
    },
    {
      name: 'withdrawStake'
      discriminator: [153, 8, 22, 138, 105, 176, 87, 66]
      accounts: [
        {
          name: 'signer'
          writable: true
          signer: true
        },
        {
          name: 'stakeVault'
          writable: true
        },
        {
          name: 'user'
          writable: true
        },
        {
          name: 'stake'
          writable: true
        },
        {
          name: 'admin'
          writable: true
        },
        {
          name: 'mint'
          writable: true
        },
        {
          name: 'fromAta'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'account'
                path: 'stakeVault'
              },
              {
                kind: 'account'
                path: 'tokenProgram'
              },
              {
                kind: 'account'
                path: 'mint'
              }
            ]
            program: {
              kind: 'const'
              value: [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          name: 'toAta'
          writable: true
          pda: {
            seeds: [
              {
                kind: 'account'
                path: 'signer'
              },
              {
                kind: 'account'
                path: 'tokenProgram'
              },
              {
                kind: 'account'
                path: 'mint'
              }
            ]
            program: {
              kind: 'const'
              value: [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          name: 'tokenProgram'
          address: 'TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb'
        },
        {
          name: 'associatedTokenProgram'
          address: 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'
        },
        {
          name: 'systemProgram'
          address: '11111111111111111111111111111111'
        }
      ]
      args: []
    },
    {
      name: 'withdrawV1'
      discriminator: [212, 118, 210, 5, 187, 1, 117, 222]
      accounts: [
        {
          name: 'signer'
          writable: true
          signer: true
        },
        {
          name: 'vault'
          writable: true
        },
        {
          name: 'userPosition'
          writable: true
        },
        {
          name: 'vaultTokenAccount'
          writable: true
        },
        {
          name: 'userTokenAccount'
          writable: true
        },
        {
          name: 'associatedTokenProgram'
          address: 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'
        },
        {
          name: 'systemProgram'
          address: '11111111111111111111111111111111'
        },
        {
          name: 'tokenProgram'
          address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'
        }
      ]
      args: [
        {
          name: 'positionIndex'
          type: 'u8'
        }
      ]
    }
  ]
  accounts: [
    {
      name: 'collection'
      discriminator: [48, 160, 232, 205, 191, 207, 26, 141]
    },
    {
      name: 'market'
      discriminator: [219, 190, 213, 55, 0, 227, 198, 154]
    },
    {
      name: 'marketV2'
      discriminator: [27, 60, 50, 75, 191, 193, 86, 227]
    },
    {
      name: 'stakeV2'
      discriminator: [207, 98, 130, 13, 118, 181, 238, 47]
    },
    {
      name: 'stakeVault'
      discriminator: [192, 112, 65, 125, 129, 151, 173, 226]
    },
    {
      name: 'user'
      discriminator: [159, 117, 95, 227, 239, 151, 58, 236]
    },
    {
      name: 'userPosition'
      discriminator: [251, 248, 209, 245, 83, 234, 17, 27]
    },
    {
      name: 'userTrade'
      discriminator: [149, 190, 47, 218, 136, 9, 222, 222]
    },
    {
      name: 'vault'
      discriminator: [211, 8, 232, 43, 2, 152, 117, 119]
    }
  ]
  events: [
    {
      name: 'marketUpdate'
      discriminator: [170, 101, 124, 32, 249, 253, 251, 96]
    },
    {
      name: 'orderUpdate'
      discriminator: [97, 239, 148, 96, 83, 234, 245, 14]
    },
    {
      name: 'priceUpdate'
      discriminator: [222, 51, 180, 226, 165, 188, 203, 54]
    },
    {
      name: 'stakeRewards'
      discriminator: [236, 217, 227, 239, 6, 129, 188, 218]
    }
  ]
  errors: [
    {
      code: 6000
      name: 'unauthorized'
      msg: 'Unauthorized access'
    },
    {
      code: 6001
      name: 'invalidPosition'
      msg: 'Invalid position'
    },
    {
      code: 6002
      name: 'invalidWithdrawAmount'
      msg: 'Invalid withdraw amount'
    },
    {
      code: 6003
      name: 'stakeLocked'
      msg: 'Stake is locked'
    },
    {
      code: 6004
      name: 'insufficientFunds'
      msg: 'Insufficient funds'
    },
    {
      code: 6005
      name: 'noRewardsAvailable'
      msg: 'No rewards available'
    },
    {
      code: 6006
      name: 'invalidPrice'
      msg: 'Invalid price'
    },
    {
      code: 6007
      name: 'noAvailableOrderSlot'
      msg: 'No available order slot'
    },
    {
      code: 6008
      name: 'marketInactive'
      msg: 'Market is inactive'
    },
    {
      code: 6009
      name: 'orderNotFound'
      msg: 'Order not found'
    },
    {
      code: 6010
      name: 'questionPeriodNotStarted'
      msg: 'Question period not started'
    },
    {
      code: 6011
      name: 'questionPeriodEnded'
      msg: 'Question period ended'
    },
    {
      code: 6012
      name: 'stakeVaultLocked'
      msg: 'Stake vault is locked'
    },
    {
      code: 6013
      name: 'marketStillActive'
      msg: 'Market still active'
    },
    {
      code: 6014
      name: 'orderNotOpen'
      msg: 'Order not open'
    },
    {
      code: 6015
      name: 'hasOpenedOrders'
      msg: 'Has opened orders'
    },
    {
      code: 6016
      name: 'insufficientLiquidity'
      msg: 'Insufficient liquidity'
    },
    {
      code: 6017
      name: 'marketNotResolved'
      msg: 'Market not resolved'
    },
    {
      code: 6018
      name: 'marketAlreadyResolved'
      msg: 'Market already resolved'
    },
    {
      code: 6019
      name: 'concurrentTransaction'
      msg: 'Concurrent transaction'
    }
  ]
  types: [
    {
      name: 'addLiquidityArgs'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'amount'
            type: 'u64'
          },
          {
            name: 'direction'
            type: {
              defined: {
                name: 'orderDirection'
              }
            }
          }
        ]
      }
    },
    {
      name: 'claimStakeRewardsArgs'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'rank'
            type: 'u16'
          },
          {
            name: 'collections'
            type: 'u8'
          }
        ]
      }
    },
    {
      name: 'collection'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'authority'
            type: 'pubkey'
          },
          {
            name: 'bump'
            type: 'u8'
          },
          {
            name: 'symbol'
            type: 'string'
          },
          {
            name: 'minted'
            type: 'u64'
          },
          {
            name: 'supply'
            type: 'u64'
          },
          {
            name: 'padding'
            type: {
              array: ['u8', 64]
            }
          }
        ]
      }
    },
    {
      name: 'createCollectionArgs'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'name'
            type: 'string'
          },
          {
            name: 'symbol'
            type: 'string'
          },
          {
            name: 'supply'
            type: 'u64'
          }
        ]
      }
    },
    {
      name: 'createUserArgs'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'name'
            type: 'string'
          }
        ]
      }
    },
    {
      name: 'initializeMarketArgs'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'name'
            type: 'string'
          },
          {
            name: 'marketId'
            type: 'u64'
          },
          {
            name: 'question'
            type: {
              array: ['u8', 80]
            }
          },
          {
            name: 'startTime'
            type: 'i64'
          },
          {
            name: 'endTime'
            type: 'i64'
          }
        ]
      }
    },
    {
      name: 'market'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'bump'
            type: 'u8'
          },
          {
            name: 'authority'
            type: 'pubkey'
          },
          {
            name: 'marketId'
            docs: ['Unique identifier for the market']
            type: 'u64'
          },
          {
            name: 'name'
            docs: ['The event being predicted (e.g., "tJUP/TRD")']
            type: 'string'
          },
          {
            name: 'hypePrice'
            docs: [
              'Current price for Hype outcome (0-1000000, representing 0 to 1 TRD)',
              '1000000 = 1 TRD, 500000 = 0.5 TRD, etc.'
            ]
            type: 'u64'
          },
          {
            name: 'flopPrice'
            docs: [
              'Current price for Flop outcome (0-1000000, representing 0 to 1 TRD)'
            ]
            type: 'u64'
          },
          {
            name: 'hypeLiquidity'
            docs: ['Total liquidity for Hype (in TRD)']
            type: 'u64'
          },
          {
            name: 'flopLiquidity'
            docs: ['Total liquidity for Flop (in TRD)']
            type: 'u64'
          },
          {
            name: 'totalHypeShares'
            docs: ['Total number of Hype shares issued']
            type: 'u64'
          },
          {
            name: 'totalFlopShares'
            docs: ['Total number of Flop shares issued']
            type: 'u64'
          },
          {
            name: 'totalVolume'
            docs: ['Total trading volume (in TRD) for all resolutions']
            type: 'u64'
          },
          {
            name: 'mint'
            docs: ['Mint $TRD token']
            type: 'pubkey'
          },
          {
            name: 'ts'
            docs: ['Timestamp of the init']
            type: 'i64'
          },
          {
            name: 'updateTs'
            type: 'i64'
          },
          {
            name: 'openOrdersCount'
            docs: ['Total number of open orders in this market']
            type: 'u64'
          },
          {
            name: 'nextOrderId'
            docs: ['Next available order ID']
            type: 'u64'
          },
          {
            name: 'feeBps'
            docs: ['Fees applied to trades (in basis points, e.g., 2.131% fee)']
            type: 'u16'
          },
          {
            name: 'feeVault'
            docs: ['Vault to Receive fees']
            type: 'pubkey'
          },
          {
            name: 'isActive'
            docs: ['Whether the market is currently active for trading']
            type: 'bool'
          },
          {
            name: 'marketPrice'
            type: 'u64'
          },
          {
            name: 'previousResolvedQuestion'
            type: {
              defined: {
                name: 'resolvedQuestion'
              }
            }
          },
          {
            name: 'currentQuestionId'
            docs: [
              'Index of the current week in the weekly_results array initialized with default values'
            ]
            type: 'u64'
          },
          {
            name: 'currentQuestionStart'
            type: 'i64'
          },
          {
            name: 'currentQuestionEnd'
            type: 'i64'
          },
          {
            name: 'currentQuestion'
            docs: ['The question or prediction topic for the current week']
            type: {
              array: ['u8', 80]
            }
          },
          {
            name: 'liquidity'
            type: 'u64'
          },
          {
            name: 'padding'
            type: {
              array: ['u8', 200]
            }
          }
        ]
      }
    },
    {
      name: 'marketUpdate'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'marketId'
            type: 'u64'
          },
          {
            name: 'question'
            type: 'string'
          },
          {
            name: 'startTime'
            type: 'i64'
          },
          {
            name: 'endTime'
            type: 'i64'
          },
          {
            name: 'hypeLiquidity'
            type: 'u64'
          },
          {
            name: 'flopLiquidity'
            type: 'u64'
          },
          {
            name: 'winningDirection'
            type: {
              defined: {
                name: 'winningDirection'
              }
            }
          },
          {
            name: 'finalHypePrice'
            type: 'u64'
          },
          {
            name: 'finalFlopPrice'
            type: 'u64'
          },
          {
            name: 'timestamp'
            type: 'i64'
          },
          {
            name: 'totalHypeShares'
            type: 'u64'
          },
          {
            name: 'totalFlopShares'
            type: 'u64'
          }
        ]
      }
    },
    {
      name: 'marketV2'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'bump'
            type: 'u8'
          },
          {
            name: 'authority'
            type: 'pubkey'
          },
          {
            name: 'marketId'
            docs: ['Unique identifier for the market']
            type: 'u64'
          },
          {
            name: 'hypePrice'
            type: 'u64'
          },
          {
            name: 'flopPrice'
            type: 'u64'
          },
          {
            name: 'hypeLiquidity'
            docs: ['Total liquidity for Hype (in TRD)']
            type: 'u64'
          },
          {
            name: 'flopLiquidity'
            docs: ['Total liquidity for Flop (in TRD)']
            type: 'u64'
          },
          {
            name: 'hypeShares'
            docs: ['Total number of Hype shares issued']
            type: 'u64'
          },
          {
            name: 'flopShares'
            docs: ['Total number of Flop shares issued']
            type: 'u64'
          },
          {
            name: 'volume'
            docs: ['Total trading volume (in TRD)']
            type: 'u64'
          },
          {
            name: 'mint'
            docs: ['Mint $TRD token']
            type: 'pubkey'
          },
          {
            name: 'updateTs'
            type: 'i64'
          },
          {
            name: 'openedOrders'
            docs: ['Total number of open orders in this market']
            type: 'u64'
          },
          {
            name: 'nextOrderId'
            docs: ['Next available order ID']
            type: 'u64'
          },
          {
            name: 'feeBps'
            docs: ['Fees applied to trades (in basis points, e.g., 2.131% fee)']
            type: 'u16'
          },
          {
            name: 'nftHoldersFeeAvailable'
            type: 'u64'
          },
          {
            name: 'nftHoldersFeeClaimed'
            type: 'u64'
          },
          {
            name: 'marketFeeAvailable'
            type: 'u64'
          },
          {
            name: 'marketFeeClaimed'
            type: 'u64'
          },
          {
            name: 'isActive'
            docs: ['Whether the market is currently active for trading']
            type: 'bool'
          },
          {
            name: 'marketStart'
            docs: [
              'Index of the current week in the weekly_results array initialized with default values'
            ]
            type: 'i64'
          },
          {
            name: 'marketEnd'
            type: 'i64'
          },
          {
            name: 'question'
            docs: ['The question or prediction topic for the current week']
            type: {
              array: ['u8', 80]
            }
          },
          {
            name: 'winningDirection'
            type: {
              defined: {
                name: 'winningDirection'
              }
            }
          },
          {
            name: 'marketLiquidityAtStart'
            type: 'u64'
          },
          {
            name: 'padding'
            type: {
              array: ['u8', 92]
            }
          }
        ]
      }
    },
    {
      name: 'openOrderArgs'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'amount'
            type: 'u64'
          },
          {
            name: 'direction'
            type: {
              defined: {
                name: 'orderDirection'
              }
            }
          }
        ]
      }
    },
    {
      name: 'order'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'ts'
            type: 'i64'
          },
          {
            name: 'orderId'
            type: 'u64'
          },
          {
            name: 'questionId'
            type: 'u64'
          },
          {
            name: 'marketId'
            type: 'u64'
          },
          {
            name: 'status'
            type: {
              defined: {
                name: 'orderStatus'
              }
            }
          },
          {
            name: 'price'
            docs: ['The price of the order (in TRD)']
            type: 'u64'
          },
          {
            name: 'totalAmount'
            docs: ['The total amount of TRD committed to this order']
            type: 'u64'
          },
          {
            name: 'totalShares'
            docs: ['The total number of shares to be purchased']
            type: 'u64'
          },
          {
            name: 'orderType'
            type: {
              defined: {
                name: 'orderType'
              }
            }
          },
          {
            name: 'direction'
            type: {
              defined: {
                name: 'orderDirection'
              }
            }
          },
          {
            name: 'padding'
            type: {
              array: ['u8', 32]
            }
          }
        ]
      }
    },
    {
      name: 'orderDirection'
      type: {
        kind: 'enum'
        variants: [
          {
            name: 'hype'
          },
          {
            name: 'flop'
          }
        ]
      }
    },
    {
      name: 'orderStatus'
      type: {
        kind: 'enum'
        variants: [
          {
            name: 'init'
          },
          {
            name: 'open'
          },
          {
            name: 'closed'
          },
          {
            name: 'claimed'
          },
          {
            name: 'liquidated'
          }
        ]
      }
    },
    {
      name: 'orderType'
      type: {
        kind: 'enum'
        variants: [
          {
            name: 'market'
          },
          {
            name: 'limit'
          }
        ]
      }
    },
    {
      name: 'orderUpdate'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'user'
            type: 'pubkey'
          },
          {
            name: 'marketId'
            type: 'u64'
          },
          {
            name: 'questionId'
            type: 'u64'
          },
          {
            name: 'orderId'
            type: 'u64'
          },
          {
            name: 'direction'
            type: {
              defined: {
                name: 'orderDirection'
              }
            }
          },
          {
            name: 'orderType'
            type: {
              defined: {
                name: 'orderType'
              }
            }
          },
          {
            name: 'orderStatus'
            type: {
              defined: {
                name: 'orderStatus'
              }
            }
          },
          {
            name: 'price'
            type: 'u64'
          },
          {
            name: 'totalShares'
            type: 'u64'
          },
          {
            name: 'totalAmount'
            type: 'u64'
          },
          {
            name: 'refundAmount'
            type: {
              option: 'u64'
            }
          },
          {
            name: 'pnl'
            type: 'i64'
          },
          {
            name: 'timestamp'
            type: 'i64'
          },
          {
            name: 'isQuestionWinner'
            type: {
              option: 'bool'
            }
          }
        ]
      }
    },
    {
      name: 'position'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'amount'
            type: 'u64'
          },
          {
            name: 'entryPrice'
            type: 'u64'
          },
          {
            name: 'ts'
            type: 'i64'
          },
          {
            name: 'isLong'
            type: 'bool'
          },
          {
            name: 'isOpen'
            type: 'bool'
          },
          {
            name: 'pnl'
            type: 'i64'
          }
        ]
      }
    },
    {
      name: 'priceUpdate'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'marketId'
            type: 'u64'
          },
          {
            name: 'hypePrice'
            type: 'u64'
          },
          {
            name: 'flopPrice'
            type: 'u64'
          },
          {
            name: 'direction'
            type: {
              defined: {
                name: 'orderDirection'
              }
            }
          },
          {
            name: 'timestamp'
            type: 'i64'
          }
        ]
      }
    },
    {
      name: 'resolvedQuestion'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'questionId'
            type: 'u64'
          },
          {
            name: 'question'
            docs: ['The question or prediction topic for this week']
            type: {
              array: ['u8', 80]
            }
          },
          {
            name: 'startTime'
            docs: ['Start timestamp of the week']
            type: 'i64'
          },
          {
            name: 'endTime'
            docs: ['End timestamp of the week']
            type: 'i64'
          },
          {
            name: 'hypeLiquidity'
            docs: ['Total liquidity for Hype (in TRD)']
            type: 'u64'
          },
          {
            name: 'flopLiquidity'
            docs: ['Total liquidity for Flop (in TRD)']
            type: 'u64'
          },
          {
            name: 'winningDirection'
            docs: ['The winning direction (Hype, Flop or None)']
            type: {
              defined: {
                name: 'winningDirection'
              }
            }
          },
          {
            name: 'marketPrice'
            type: 'u64'
          },
          {
            name: 'finalHypePrice'
            docs: ['Final price for Hype outcome at the end of the week']
            type: 'u64'
          },
          {
            name: 'finalFlopPrice'
            docs: ['Final price for Flop outcome at the end of the week']
            type: 'u64'
          },
          {
            name: 'totalHypeShares'
            docs: ['Total number of Hype shares issued']
            type: 'u64'
          },
          {
            name: 'totalFlopShares'
            docs: ['Total number of Flop shares issued']
            type: 'u64'
          },
          {
            name: 'padding'
            type: {
              array: ['u8', 40]
            }
          }
        ]
      }
    },
    {
      name: 'stakeRewards'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'user'
            type: 'pubkey'
          },
          {
            name: 'mint'
            type: 'pubkey'
          },
          {
            name: 'amount'
            type: 'u64'
          },
          {
            name: 'timestamp'
            type: 'i64'
          },
          {
            name: 'rank'
            type: 'u16'
          }
        ]
      }
    },
    {
      name: 'stakeTokenArgs'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'stakeVault'
            type: 'string'
          },
          {
            name: 'name'
            type: 'string'
          },
          {
            name: 'amount'
            type: 'u64'
          }
        ]
      }
    },
    {
      name: 'stakeV2'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'bump'
            type: 'u8'
          },
          {
            name: 'authority'
            type: 'pubkey'
          },
          {
            name: 'initTs'
            type: 'i64'
          },
          {
            name: 'withdrawTs'
            type: 'i64'
          },
          {
            name: 'claimedTs'
            type: 'i64'
          },
          {
            name: 'name'
            type: 'string'
          },
          {
            name: 'mint'
            type: 'pubkey'
          },
          {
            name: 'boost'
            type: 'bool'
          },
          {
            name: 'stakeVault'
            type: 'pubkey'
          },
          {
            name: 'claimed'
            type: 'u64'
          },
          {
            name: 'available'
            type: 'u64'
          },
          {
            name: 'amount'
            type: 'u64'
          }
        ]
      }
    },
    {
      name: 'stakeVault'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'bump'
            type: 'u8'
          },
          {
            name: 'authority'
            type: 'pubkey'
          },
          {
            name: 'initTs'
            type: 'i64'
          },
          {
            name: 'endTs'
            type: 'i64'
          },
          {
            name: 'amount'
            type: 'u64'
          },
          {
            name: 'amountPaid'
            type: 'u64'
          },
          {
            name: 'tokenDecimals'
            type: 'u8'
          },
          {
            name: 'nftStaked'
            type: 'u64'
          },
          {
            name: 'slots'
            type: 'u64'
          },
          {
            name: 'isLocked'
            type: 'bool'
          },
          {
            name: 'name'
            type: 'string'
          },
          {
            name: 'collection'
            type: 'string'
          },
          {
            name: 'tokenMint'
            type: 'pubkey'
          },
          {
            name: 'week'
            type: 'u8'
          },
          {
            name: 'tokenStaked'
            type: 'u64'
          },
          {
            name: 'sumAllUsers'
            type: 'f64'
          },
          {
            name: 'padding'
            type: {
              array: ['u8', 32]
            }
          }
        ]
      }
    },
    {
      name: 'updateStakeVaultArgs'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'amount'
            type: {
              option: 'u64'
            }
          },
          {
            name: 'isLocked'
            type: {
              option: 'bool'
            }
          },
          {
            name: 'stakeVault'
            type: 'string'
          }
        ]
      }
    },
    {
      name: 'user'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'ts'
            type: 'i64'
          },
          {
            name: 'authority'
            type: 'pubkey'
          },
          {
            name: 'bump'
            type: 'u8'
          },
          {
            name: 'referral'
            type: 'pubkey'
          },
          {
            name: 'referred'
            type: 'i64'
          },
          {
            name: 'name'
            type: 'string'
          },
          {
            name: 'swaps'
            type: 'i16'
          },
          {
            name: 'swapsMade'
            type: 'i16'
          },
          {
            name: 'staked'
            type: 'u64'
          },
          {
            name: 'firstSwap'
            type: 'i64'
          },
          {
            name: 'userTrade'
            type: 'pubkey'
          }
        ]
      }
    },
    {
      name: 'userPosition'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'ts'
            type: 'i64'
          },
          {
            name: 'bump'
            type: 'u8'
          },
          {
            name: 'totalDeposited'
            type: 'u64'
          },
          {
            name: 'totalWithdrawn'
            type: 'u64'
          },
          {
            name: 'lpShare'
            type: 'u64'
          },
          {
            name: 'totalPositions'
            type: 'u16'
          },
          {
            name: 'ticker'
            type: 'pubkey'
          },
          {
            name: 'authority'
            type: 'pubkey'
          },
          {
            name: 'positions'
            type: {
              array: [
                {
                  defined: {
                    name: 'position'
                  }
                },
                3
              ]
            }
          }
        ]
      }
    },
    {
      name: 'userTrade'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'bump'
            type: 'u8'
          },
          {
            name: 'authority'
            type: 'pubkey'
          },
          {
            name: 'totalDeposits'
            docs: [
              'The total value of deposits the user has made (in TRD)',
              'precision: QUOTE_PRECISION'
            ]
            type: 'u64'
          },
          {
            name: 'totalWithdraws'
            docs: [
              'The total value of withdrawals the user has made (in TRD)',
              'precision: QUOTE_PRECISION'
            ]
            type: 'u64'
          },
          {
            name: 'openedOrders'
            docs: ['The number of orders the user has opened']
            type: 'u64'
          },
          {
            name: 'orders'
            type: {
              array: [
                {
                  defined: {
                    name: 'order'
                  }
                },
                10
              ]
            }
          },
          {
            name: 'padding'
            type: {
              array: ['u8', 32]
            }
          }
        ]
      }
    },
    {
      name: 'vault'
      type: {
        kind: 'struct'
        fields: [
          {
            name: 'bump'
            type: 'u8'
          },
          {
            name: 'authority'
            type: 'pubkey'
          },
          {
            name: 'name'
            type: 'string'
          },
          {
            name: 'tokenAccount'
            type: 'pubkey'
          },
          {
            name: 'tickerAddress'
            type: 'pubkey'
          },
          {
            name: 'totalDeposited'
            type: 'u64'
          },
          {
            name: 'totalWithdrawn'
            type: 'u64'
          },
          {
            name: 'initTs'
            type: 'i64'
          },
          {
            name: 'netDeposits'
            type: 'u128'
          },
          {
            name: 'netWithdraws'
            type: 'u128'
          },
          {
            name: 'longBalance'
            type: 'u64'
          },
          {
            name: 'shortBalance'
            type: 'u64'
          },
          {
            name: 'longPositionsOpened'
            type: 'u64'
          },
          {
            name: 'shortPositionsOpened'
            type: 'u64'
          }
        ]
      }
    },
    {
      name: 'winningDirection'
      type: {
        kind: 'enum'
        variants: [
          {
            name: 'none'
          },
          {
            name: 'hype'
          },
          {
            name: 'flop'
          }
        ]
      }
    }
  ]
}
