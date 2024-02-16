// pub fn main() {
//     let n = sp1_zkvm::io::read::<u32>();

//     let mut a = 0;
//     let mut b = 1;
//     let mut sum;
//     for _ in 1..n {
//         sum = a + b;
//         a = b;
//         b = sum;
//     }

//     sp1_zkvm::io::write(&a);
//     sp1_zkvm::io::write(&b);
// }

#![no_main]
sp1_zkvm::entrypoint!(main);

// use anyhow::Result;
// use alloy_primitives::Address;
use ed25519_dalek::*;
use hex::decode;
use serde::{Deserialize, Serialize};
use std::hint::black_box;

#[derive(Serialize, Deserialize)]
struct Transaction {
    pub_key: [u8; 32], // TODO use alloy
    sig: String,       //[u8; 64],     // TODO use alloy
    data: String,      //[u8],
}

pub fn main() {
    let tx = sp1_zkvm::io::read::<Transaction>();
    verify_sig(tx).unwrap();
    println!("done");
}

fn verify_sig(tx: Transaction) -> Result<(), ed25519_dalek::SignatureError> {
    let pub_key = black_box(VerifyingKey::from_bytes(&tx.pub_key).unwrap());
    let msg = decode(&tx.data).unwrap();
    let sig = black_box(Signature::try_from(&decode(&tx.sig).unwrap()[..]).unwrap());
    pub_key.verify_strict(&black_box(msg), &black_box(sig))
}
