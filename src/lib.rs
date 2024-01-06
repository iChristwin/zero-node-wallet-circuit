pub mod circuits;
pub mod generator;

use generator::{gen_pk, gen_proof, gen_srs};
use halo2_curves::bn256::Fr;
use halo2_curves::serde::SerdeObject;
use halo2_proofs::circuit::Value;
use snark_verifier::loader::evm::encode_calldata;
use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn prove(input: &str) -> std::string::String {
    // TODO parse input json, like {"private_a": 3, "private_b": 4}
    let input_v: JsonValue = serde_json::from_str(&input).unwrap();
    let item_str = &input_v.as_array().unwrap()[0].as_str().unwrap();

    let v: JsonValue = serde_json::from_str(item_str).unwrap();
    let address_str = v["wallet_address"]
        .as_str()
        .unwrap()
        .strip_prefix("0x")
        .unwrap();
    let address = hex::decode(address_str).unwrap();
    let address =
        Fr::from_raw_bytes_unchecked(&[vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], address].concat());

    let params = gen_srs(4);

    // TODO replace your circuit struct
    let circuit = crate::circuits::wallet::WalletCircuit {
        wallet_address: Value::known(address),
        _marker: PhantomData,
    };

    let pk = gen_pk(&params, &circuit);

    // TODO public info
    let c = address;
    let instances = vec![vec![c]];

    let proof = gen_proof(&params, &pk, circuit.clone(), &instances);
    let calldata = encode_calldata(&instances, &proof);

    format!(
        r#"{{
            "proof": "0x{}",
            "calldata": "0x{}"
        }}"#,
        hex::encode(&proof),
        hex::encode(calldata),
    )
}
