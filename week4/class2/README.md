
## Setup your development environment 

Follow the steps [here](https://docs.arbitrum.io/stylus/quickstart#setting-up-your-development-environment) to set up your development environment.


## Guide to week 4 class 2 HOMEWORK (Performance: Keccak vs Mersenne Twister)
Performance Comparison: Monte Carlo Option Pricing, Keccak vs Mersenne Twister version.

Your goal is to test both versions and determine which one can run more simulations before hitting the gas limit. Increase the number of simulations by `5,000` each time you run test and see which version reaches the limit first.


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