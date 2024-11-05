<b> Market Simulation </b>

- Initial Market -

Hype Liquidity = 1000
Flop Liquidity = 1000

Hype Shares = 0
Flop Shares = 0

Hype Price = 0.5
Flop Price = 0.5

- New Order (Hype) -

Amount = 10 $TRD

// new_directional_liquidity = hype_liquidity + amount
Directional Liquidity = 1000 + 10 = 1010 $TRD

// buy_price = new_directional_liquidity / (new_directional_liquidity + flop_liquidity)
Buy Price = 1010 / (1010 + 1000) = 0.5024875622 $TRD

// shares = amount / buy_price
Shares To Receive = 10 / 0.5024875622 = 19.9009900986 SHARES

- Makert Updated -

Hype Liquidity = 1010 $TRD
Flop Liquidity = 1000 $TRD

Hype Shares = 19.9009900986
Flop Shares = 0

Hype Price = 0.5024875622 $TRD
Flop Price = 0.4975124378 $TRD

- New Order (Flop) -

Amount = 1000 $TRD

// new_directional_liquidity = flop_liquidity + amount
Directional Liquidity = 1000 + 1000 = 2000 $TRD

// buy_price = new_directional_liquidity / (new_directional_liquidity + hype_liquidity)
Buy Price = 2000 / (2000 + 1010) = 0.6644518272 $TRD

// shares = amount / buy_price
Shares To Receive = 1000 / 0.6644518272 = 1505.00000009632 SHARES

- Makert Updated -

Hype Liquidity = 1010 $TRD
Flop Liquidity = 2000 $TRD

Hype Shares = 19.9009900986
Flop Shares = 1505.00000009632

Hype Price = 0.3355481728 $TRD
Flop Price = 0.6644518272 $TRD

- Market ended -

Hype Win

// med_price = new_opposit_liqidity / market_shares;
Med Price = 1000 / 19.9009900986 = 50.2487562199

// user_payout_by_liquidity = (shares - amount) x med_price + amount;
User Payout By Liquidity = (19.9009900986 - 10) x 50.2487562199 + 10 = 507.5124378002 $TRD

// user_payout_by_shate = shares x (1 - price_per_share) + amount
User Payout = 19.9009900986 x (1 - 0.5024875622) + 10 = 19.9009900986 $TRD

Flop Win

// med_price = market_opposit_liquidity / market_shares;
Med Price = 1010 / 1505.00000009632 = 0.6710963455

// user_payout_by_liquidity = (shares - amount) x med_price + amount;
User Payout By Liquidity = (1505.00000009632 - 1000) x 0.6710963455 + 1000 = 1338.9036545421 $TRD

// user_payout_by_share = shares x (1 - price_per_share) + amount
User Payout = 1505.00000009632 x (1 - 0.6644518272) + 1000 = 1505.00000009632 $TRD
