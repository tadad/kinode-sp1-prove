use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};

use kinode_process_lib::{call_init, println, Address, vfs::{create_drive, open_file}};

const ELF: &[u8] = include_bytes!("../../../program/elf/riscv32im-succinct-zkvm-elf");

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
    let n = 500u32;
    stdin.write(&n);
    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // Read output.
    let a = proof.stdout.read::<u32>();
    let b = proof.stdout.read::<u32>();
    println!("a: {}", a);
    println!("b: {}", b);

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // Return proof.
    serde_json::to_vec(&proof).unwrap()
}
