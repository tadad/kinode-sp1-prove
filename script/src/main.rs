use alloy_core::primitives::{Address, Signature, U256};
use serde::{Deserialize, Serialize};
use sp1_core::{utils, SP1Prover, SP1Stdin, SP1Verifier};

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

/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_bytes!("../../execution_engine/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    let mut stdin = SP1Stdin::new();
    let tx = Transaction {
        from: Address::ZERO,
        sig: Signature::test_signature(),
        data: TxType::Bridge {
            owner: Address::ZERO,
            id: U256::from(1),
        },
    };

    stdin.write(&tx);
    println!("before prove");

    // Generate the proof for the given program.
    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");
    println!("after prove");
    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");
    println!("after verify");
    // Save the proof.
    proof
        .save("proof-with-pis.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!")
}
