#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_core::primitives::{Address, Signature, U256};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
// use sha2::{Digest, Sha256};
// use std::hash::{Hash, Hasher};
// use ed25519_dalek::*;
// use hex::decode;
// use std::hint::black_box;

// TODO need a way to hash this into a single U256 value
#[derive(Serialize, Deserialize)]
struct State {
    data: HashMap<U256, String>,               // NFT id to metadata
    accounts: HashMap<Address, HashSet<U256>>, // pub_key to NFTs owned
}

// TODO need a better way to hash the state
// impl Hash for State {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         let mut data_vec: Vec<_> = self.data.iter().collect();
//         data_vec.sort_by(|a, b| a.0.cmp(b.0));
//         for (key, value) in data_vec {
//             key.hash(state);
//             value.hash(state);
//         }

//         let mut accounts_vec: Vec<_> = self.accounts.iter().collect();
//         accounts_vec.sort_by(|a, b| a.0.cmp(b.0));
//         for (key, value_set) in accounts_vec {
//             key.hash(state);
//             let mut value_vec: Vec<_> = value_set.iter().collect();
//             value_vec.sort();
//             value_vec.hash(state);
//         }
//     }
// }

#[derive(Serialize, Deserialize)]
struct Transaction {
    from: Address,
    sig: Signature,
    data: TxType,
}

#[derive(Serialize, Deserialize)]
enum TxType {
    Bridge {
        owner: Address,
        id: U256,
    },
    Transfer {
        from: Address,
        to: Address,
        id: U256,
    },
    Update {
        id: U256,
        metadata: String,
    },
    Withdraw(U256),
}

pub fn main() {
    let mut state = sp1_zkvm::io::read::<State>();
    let tx = sp1_zkvm::io::read::<Transaction>();

    // match verify_sig(tx) {
    //     Ok(_) => sp1_zkvm::io::write(&"ok"),
    //     Err(_) => sp1_zkvm::io::write(&"fail"),
    // }
    match tx.data {
        TxType::Bridge { owner, id } => {
            let nfts = state.accounts.entry(owner).or_insert_with(HashSet::new);
            nfts.insert(id);
            sp1_zkvm::io::write(&state);
        }
        TxType::Update { id, metadata } => {
            state.data.insert(id, metadata);
        }
        TxType::Transfer { from, to, id } => {
            let from_set = state.accounts.get_mut(&from).unwrap();
            from_set.remove(&id);
            let to_set = state.accounts.get_mut(&to).unwrap();
            to_set.insert(id);
        }
        TxType::Withdraw(id) => {
            let nfts = state.accounts.entry(tx.from).or_insert_with(HashSet::new);
            nfts.remove(&id);
        }
    }

    sp1_zkvm::io::write(&state);
    println!("done");
}

// fn verify_sig(tx: Transaction) -> Result<(), ed25519_dalek::SignatureError> {
//     let pub_key = black_box(VerifyingKey::from_bytes(&tx.pub_key).unwrap());
//     let msg = decode(&tx.data).unwrap();
//     let sig = black_box(Signature::try_from(&decode(&tx.sig).unwrap()[..]).unwrap());
//     pub_key.verify_strict(&black_box(msg), &black_box(sig))
// }
