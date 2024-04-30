use core::str;

use methods::ZKAF_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde::{Deserialize, Serialize};


use tlsn_substrings_verifier::{self, proof::{SessionHeader, SubstringsProof}};
#[derive(Serialize, Deserialize, Debug)]
struct ZkParam {
    header: SessionHeader,
    substrings: SubstringsProof,
}

fn main() {
    // Obtain the default prover.
    let prover = default_prover();

    // read in th einputs from json
    let proof_params = std::fs::read_to_string("./host/fixtures/zk_params.json").unwrap();
    let proof_params: ZkParam = serde_json::from_str(proof_params.as_str()).unwrap();

    // pass the input to the guest code
    let input: (SessionHeader, SubstringsProof) = (proof_params.header, proof_params.substrings);
    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, ZKAF_ELF).unwrap();

    // Extract journal of receipt
    let (request, response): (bool, bool)= receipt.journal.decode().unwrap();

    // Print, notice, after committing to a journal, the private input became public
    println!("I generated a proof of guest execution!");
    println!("Request Verified: \n{}", request);
    println!("Response Verified: \n {}", response);
}
