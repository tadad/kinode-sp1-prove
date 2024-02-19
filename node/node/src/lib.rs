use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use tempfile::tempfile_in;

use hex_literal::hex;
use kinode_process_lib::{
    await_message, call_init, println,
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
    println!("prove_fib: begin3");

    let temp_dir = std::env::var("TEMP_DIR").unwrap();
    let mut temp_file = tempfile_in(temp_dir).unwrap();

    // Write some text to the temporary file.
    let written_text = "Hello, world!";
    write!(temp_file, "{}", written_text).expect("Failed to write to temporary file");

    // Go back to the start of the file before reading.
    temp_file
        .seek(SeekFrom::Start(0))
        .expect("Failed to seek to start of file");

    // Read the text back from the temporary file.
    let mut buffer = String::new();
    temp_file
        .read_to_string(&mut buffer)
        .expect("Failed to read from temporary file");

    // Print the text that was read.
    println!("Read from temporary file: {}", buffer);

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
    println!("before prove");
    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // Verify proof.
    println!("before verify");
    SP1Verifier::verify(ELF, &proof).expect("verification failed");
    println!("after verify");
    // Return proof.
    println!("prover: verified proof");
    serde_json::to_vec(&proof).unwrap()
}
