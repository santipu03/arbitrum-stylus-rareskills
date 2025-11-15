
## Setup your development environment 

Follow the steps [here](https://docs.arbitrum.io/stylus/quickstart#setting-up-your-development-environment) to set up your development environment.


## Guide to week 3 class 2 HOMEWORK (Gas Cost: Stylus vs Solidity)
Gas Cost Comparison: Stylus vs Solidity (5000th Prime)

Your goal is, you will run the same “nth prime” computation in:

- A Stylus contract, and

- A Solidity contract on Remix

to see which environment hits gas limits / runs out of gas first when asking for the `5000`-th prime



### Test Stylus contract

1. Rename `.env.example` to `.env` in `./scripts` folder.

2. Run the following command to deploy the contract
    ```
    ./scripts/deploy.sh
    ```
    Upon successful deployment, set the `CONTRACT_ADDRESS` in your `.env` file to the address of the deployed contract.

3. Run the following command to run the test script
    ```
    ./scripts/test.sh
    ```