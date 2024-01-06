# Wallet Verification Circuit

This code implements a Halo 2 circuit for verifying a wallet address.

**Key Features:**

- Uses a `WalletChip` to encapsulate wallet-related constraints.
- Defines a `WalletCircuit` to represent the overall verification logic.
- Includes a test case to demonstrate usage.

**Usage:**

1. Install the required dependencies:

    ```bash
    cargo install halo2-proofs
    ```

2. build wasm

    ```shell
    wasm-pack build --target nodejs --out-dir pkg
    ```

you will find the wasm file in the `pkg` folder.

## Build executable file

```shell
cargo build --release
```

## Generate verify smart contract

```shell
target/release/halo2-simple-circuit solidity
```

You will find `Verifier.sol` under the current folder.
Then you can deploy the smart contract to IoTeX chain or other ETH-compatible chains.

**Circuit Structure:**

- **`WalletChip`**:
  - Encapsulates the constraints for verifying a wallet address.
  - Includes a `configure` method to set up the constraints.
- **`WalletConfig`**:
  - Stores configuration data for the `WalletChip`.
- **`WalletCircuit`**:
  - Represents the overall verification circuit.
  - Implements the `Circuit` trait for integration with Halo 2.
  - Includes methods for configuration and synthesis.
- **`tests` module:**
  - Contains a test case to verify a sample wallet address.

**Additional Notes:**

- The code uses a `MockProver` for testing purposes.
- For real-world usage, you'll need to use a proper Halo 2 prover.
