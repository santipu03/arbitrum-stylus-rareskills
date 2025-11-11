
## Setup your development environment 

Follow the steps [here](https://docs.arbitrum.io/stylus/quickstart#setting-up-your-development-environment) to set up your development environment.


## Guide to week 3 class 1 HOMEWORK (Inheritance)

This task involves filling in the missing parts of the contract in `src/lib.rs`.

Example:
```rust
pub fn my_func(&mut self, addy: /* 1______ */) {
    // Logic...
}
```

Solution:
```rust
pub fn my_func(&mut self, addy: Address) {
    // Logic...
}
```

### Guide
1. `/* 1. ______ */`

    Hint: Import `ContractB` from the module `contract_b`.

2. `/* 2. ______ */`

    Hint: Replace with the `public` attribute to make the function(s) in the impl block public.

3. `/* 3. ______ */`

    Hint: Within the `ret_num_b` function, call the `ret_num()` function defined in `contract_b`, and return its value.

4. `/* 4. ______ */`

    Hint: Implement the traits in the order, `IContractA` first, then `IContractB`, using the `implements` attribute.


## Test

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