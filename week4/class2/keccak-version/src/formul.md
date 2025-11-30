

## Simplified MC Mathematical Formula

For each simulation, we will use the following formula to calculate the **simulated future price** of the asset:

$$
S_T = S_0 \cdot e^{\sigma \cdot \sqrt{T} \cdot Z}
$$

Where:

* $S_T$: **simulated future price** of the asset
* $S_0$: starting price of the asset (`start_price`)
* $\sigma$: volatility (`vol`)
* $T$: expiration date in years (`time_to_exp`)
* $Z$: a random value drawn from a **standard normal distribution** (mean = 0, std dev = 1)



### Call Option Payoff Formula

Once you have $S_T$, the simulated price at expiration, the **payoff** for a call option is:

$$
\text{Payoff}^{(i)} = \max\left( S_T^{(i)} - K, \; 0 \right)
$$

Where:

* $K$: strike price (`strike_price`)
* If the simulated price is **above** the strike price → you make a profit
* Otherwise → option expires worthless (payoff = 0)


### Put Option Payoff Formula

Once you have $S_T$, the simulated price at expiration, the **payoff** for a put option is:

$$
\text{Payoff}^{(i)} = \max\left( K - S_T^{(i)}, \; 0 \right)
$$


