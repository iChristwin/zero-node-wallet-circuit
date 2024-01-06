use halo2_curves::ff::Field;
use halo2_proofs::{
    circuit::{Chip, Layouter, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Constraints, Instance, Selector},
    poly::Rotation,
};

use std::marker::PhantomData;

// A chip responsible for configuring and handling the wallet-related constraints in the circuit
#[derive(Debug, Clone)]
struct WalletChip<F: Field> {
    config: WalletConfig,
    _marker: PhantomData<F>,
}

impl<F: Field> WalletChip<F> {
    // Constructor method for WalletChip
    fn new(config: <Self as Chip<F>>::Config) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    // Initializes the configuration for the wallet, setting up necessary constraints
    fn initialize_chip_config(
        meta: &mut ConstraintSystem<F>,
        advice: [Column<Advice>; 1],
        instance: Column<Instance>,
    ) -> WalletConfig {
        // Enable equality for instance and advice columns
        meta.enable_equality(instance);
        for column in &advice {
            meta.enable_equality(*column);
        }
        let selector = meta.selector();

        // Create a gate named "wallet_address" to enforce constraints on wallet addresses
        meta.create_gate("wallet_address", |meta| {
            let s = meta.query_selector(selector);
            let ac = meta.query_advice(advice[0], Rotation::cur());
            Constraints::with_selector(s, vec![ac.clone() - ac])
        });

        // Return the configured WalletConfig
        WalletConfig {
            advice,
            selector,
            instance,
        }
    }
}

impl<F: Field> Chip<F> for WalletChip<F> {
    type Config = WalletConfig;
    type Loaded = ();

    // Returns the configuration of the chip
    fn config(&self) -> &Self::Config {
        &self.config
    }

    // Returns the loaded data of the chip (empty in this case)
    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

// Configuration struct holding information about wallet-related columns and selectors
#[derive(Clone, Debug)]
pub struct WalletConfig {
    pub advice: [Column<Advice>; 1],
    pub instance: Column<Instance>,
    pub selector: Selector,
}

// Main circuit struct representing a wallet-related computation
#[derive(Default, Clone)]
pub struct WalletCircuit<F: Field> {
    pub wallet_address: Value<F>,
    _marker: PhantomData<F>,
}

impl<F: Field> Circuit<F> for WalletCircuit<F> {
    type Config = WalletConfig;
    type FloorPlanner = SimpleFloorPlanner;

    // Creates a new instance of the circuit without any witnesses
    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    // Configures the circuit, setting up the necessary columns and constraints
    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let advice = [meta.advice_column()];
        let instance = meta.instance_column();

        // Initialize the wallet chip configuration
        WalletChip::initialize_chip_config(meta, advice, instance)
    }

    // Synthesizes the circuit, assigning constraints and verifying the provided data
    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), halo2_proofs::plonk::Error> {
        let wallet_chip = WalletChip::<F>::new(config);

        // Assign the "confirm_wallet" region to enforce constraints on wallet addresses
        let out = layouter.assign_region(
            || "confirm_wallet",
            |mut region| {
                let advice = wallet_chip.config.advice;
                let s = wallet_chip.config.selector;

                // Enable the selector in the region to enforce constraints
                s.enable(&mut region, 0)?;

                // Assign the wallet address based on provided data
                let wallet_address =
                    region.assign_advice(|| "address", advice[0], 0, || self.wallet_address)?;

                Ok(wallet_address)
            },
        )?;
        // Constrain the instance to match the configured instance in the wallet chip
        layouter
            .namespace(|| "out")
            .constrain_instance(out.cell(), wallet_chip.config.instance, 0)
    }
}

// Tests for verifying the functionality of the wallet circuit
#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use super::WalletCircuit;
    use halo2_curves::{bn256::Fr, serde::SerdeObject};
    use halo2_proofs::{circuit::Value, dev::MockProver};
    use hex;

    #[test]
    fn verify() {
        let k = 4;

        // Create a sample wallet address for testing
        let address_str = "0x7f9c6d42ac373f9947da39c23379b211e03f3b68"
            .strip_prefix("0x")
            .unwrap();
        let address = hex::decode(address_str).unwrap();
        let address = Fr::from_raw_bytes_unchecked(
            &[vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], address].concat(),
        );

        // Create a wallet circuit instance with a known wallet address
        let circuit = WalletCircuit {
            wallet_address: Value::known(address),
            _marker: PhantomData,
        };

        // Provide the wallet address as a public input for testing
        let public_inputs = vec![address];

        // Run the circuit through a mock prover and verify the result
        let prover = MockProver::run(k, &circuit, vec![public_inputs.clone()]).unwrap();
        assert_eq!(prover.verify(), Ok(()));
    }
}

// Entry point for the program (main function is currently empty)
fn main() {}
