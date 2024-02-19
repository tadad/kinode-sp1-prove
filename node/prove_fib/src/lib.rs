use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};

use hex_literal::hex;
use kinode_process_lib::{
    call_init, println,
    vfs::{create_drive, open_file},
    Address,
};
use serde::{Deserialize, Serialize};

const ELF: &[u8] = include_bytes!("../../../execution_engine/elf/riscv32im-succinct-zkvm-elf");

#[derive(Serialize, Deserialize)]
struct Transaction {
    pub_key: [u8; 32], // TODO use alloy
    sig: String,       //[u8; 64],     // TODO use alloy
    data: String,      //[u8],
}

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

call_init!(init);

fn init(our: Address) {
    println!("prove_fib: begin");

    let proof = prove();

    // Save proof.
    let drive_path = create_drive(our.package_id(), "proofs").unwrap();
    let proof_file = open_file(&format!("{}/proof.json", drive_path), true).unwrap();
    proof_file.write(&proof).unwrap();

    println!("succesfully generated and verified proof for the program!")
}

fn prove() -> Vec<u8> {
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    let tx = Transaction {
        pub_key: hex!("ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"),
        sig: "46557EFE96D22D07E104D9D7FAB558FB02F6B13116056E6D7C300D7BB132059907D538EAC68EC7864AA2AC2E23EA7082A04002B0ACDAC2FF8CCAD7E80E64DD00".to_string(),
        data: "616263616263616263616263616263616263616263616263616263616263616263616263616263".to_string(),
    };

    stdin.write(&tx);
    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // Return proof.
    println!("prover: verified proof");
    serde_json::to_vec(&proof).unwrap()
}
