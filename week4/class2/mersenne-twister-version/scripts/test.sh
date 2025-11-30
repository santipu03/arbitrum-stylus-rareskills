#!/bin/bash

# Load variables from .env file
set -o allexport
source scripts/.env
set +o allexport

# -------------- #
# Initial checks #
# -------------- #
if [ -z "$PRIVATE_KEY" ] || [ -z "$CONTRACT_ADDRESS" ]
then
    echo " PRIVATE_KEY or CONTRACT_ADDRESS is not set. Set them in the .env file"
    exit 0
fi



# --------------------------
# Monte Carlo Pricing Option
# --------------------------
echo ""
echo "************"
echo "Monte Carlo"
echo "************"


sc=$(cast call --rpc-url $RPC_URL $CONTRACT_ADDRESS "monteCarloOptionPrice(uint64,uint64,uint64,uint64,uint64,uint64) (uint64)" 1256690730 10 15 10000 9900 43000) 


echo "Monte Carlo: $sc"



# rand_val: 1256690730
# time_to_exp:   10 0.1 years 36 days
# vol: 15 0.15
# start_price:  10000 100
# strike_price: 9900 99
# num_simulations: 40,000




