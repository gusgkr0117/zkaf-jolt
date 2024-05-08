#![cfg_attr(feature = "guest", no_std)]
#![no_main]
use tlsn_substrings_verifier::proof::{SessionHeader, SubstringsProof};

#[jolt::provable]
fn circuit_main(session_header: String, substrings: String) -> (bool, bool) {
    // handle deserialization manually
    // ? more efficiently pass bytes instead of strings?
    let session_header: SessionHeader = serde_json::from_str(&session_header).unwrap();
    let substrings: SubstringsProof = serde_json::from_str(&substrings).unwrap();

    let (sent, recv) = substrings.verify(&session_header).unwrap();

    // Log that we've successfully recovered the request and response...
    let is_req = !sent.data().to_vec().is_empty();
    let is_res = !recv.data().to_vec().is_empty();

    (is_req, is_res)
}
