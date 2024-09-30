# Id Card

Solana based id card manager program. It uses Solana [PDA account](https://solana.com/docs/core/pda) to store Details of Person.

# Testing as developer

- Install solana cli and rust. You may following [this guide](https://solana.com/docs/intro/installation)
- To build run:
  ```sh
  cargo-build-bpf
  ```
- To deploy run
  ```sh
  solana program deploy ./target/deploy/id_card.so
  ```
- To test run
  ```sh
  PROG=<program_id> node client/create_account.js
  PROG=<program_id> node client/fetch_account.js
  ```
