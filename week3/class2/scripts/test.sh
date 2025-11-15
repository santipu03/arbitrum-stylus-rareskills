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
# Prime Number Pricing Option
# --------------------------
echo ""
echo "************"
echo "Prime Number"
echo "************"


prime=$(cast call --rpc-url $RPC_URL $CONTRACT_ADDRESS "getPrime(uint64) (uint64)" 50000) # max i have tried is 111000-th 


echo "Prime Number: $prime"



