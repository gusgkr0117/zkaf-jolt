use serde::{Deserialize, Serialize};
use tlsn_substrings_verifier::proof::{SessionHeader, SubstringsProof};

#[derive(Serialize, Deserialize, Debug)]
struct ZkParam {
    header: SessionHeader,
    substrings: SubstringsProof,
}

pub fn main() {
    let (circuit_main, verify_circuit_main) = guest::build_circuit_main();

    let proof_params = std::fs::read_to_string("inputs/zk_params.json").unwrap();
    let proof_params: ZkParam = serde_json::from_str(proof_params.as_str()).unwrap();
    let (session_header, substrings): (String, String) = (
        serde_json::to_string(&proof_params.header).unwrap(),
        serde_json::to_string(&proof_params.substrings).unwrap(),
    );

    let (output, proof) = circuit_main(session_header, substrings);
    let is_valid = verify_circuit_main(proof);

    println!("output: {:?}", output);
    println!("valid: {}", is_valid);
}
