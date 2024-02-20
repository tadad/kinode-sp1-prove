#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_core::primitives::{Address, Signature, U256};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
// use ed25519_dalek::*;
// use hex::decode;
// use std::hint::black_box;

// TODO need a way to hash this into a single U256 value
#[derive(Serialize, Deserialize)]
struct State {
    data: HashMap<U256, String>,               // NFT id to metadata
    accounts: HashMap<Address, HashSet<U256>>, // pub_key to NFTs owned
}

#[derive(Serialize, Deserialize)]
struct Withdrawals {
    // TODO should be a Merkel (Patricia) tree (trie?)
}

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
    let mut state = State {
        data: HashMap::new(),
        accounts: HashMap::new(),
    };
    let tx = sp1_zkvm::io::read::<Transaction>();

    // TODO This is expensive so skip it for now
    // match verify_sig(tx) {
    //     Ok(_) => sp1_zkvm::io::write(&"ok"),
    //     Err(_) => sp1_zkvm::io::write(&"fail"),
    // }
    match tx.data {
        TxType::Bridge { owner, id } => {
            let nfts = state.accounts.entry(owner).or_insert_with(HashSet::new);
            nfts.insert(id);
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

    // TODO probably do need to hash the state
    // then need a separate state which will allow for merkle proof-based withdrawals
    // That way we don't need to make the entire state a MPT. Just the deposits/withdrawals
    // Could easily make a library to plug and play: your state just needs to impl Hash (deterministically)
    // and then just import the deposit/withdraw library to handle ERC20/721/1155/etc
    // then you state root+d/w state root is posted to chain > BAM
    let serialized = serde_json::to_string(&state).unwrap();
    sp1_zkvm::io::write(&serialized);
}

// fn verify_sig(tx: Transaction) -> Result<(), ed25519_dalek::SignatureError> {
//     let pub_key = black_box(VerifyingKey::from_bytes(&tx.pub_key).unwrap());
//     let msg = decode(&tx.data).unwrap();
//     let sig = black_box(Signature::try_from(&decode(&tx.sig).unwrap()[..]).unwrap());
//     pub_key.verify_strict(&black_box(msg), &black_box(sig))
// }
