# kinode-prove-fib

Proving [SP1](https://github.com/succinctlabs/sp1)'s Fibonacci example in a Kinode process.

## Usage

[Install](https://succinctlabs.github.io/sp1/getting-started/install.html) and [Quickstart](https://succinctlabs.github.io/sp1/getting-started/quickstart.html) SP1 up to [Build Program](https://succinctlabs.github.io/sp1/getting-started/quickstart.html#build-program).

To generate the proof inside of Kinode:

```
# Terminal 1: Start a fake Kinode
kit f

# Terminal 2: Prove it
kit bs prove_fib
```

You should see your fake Kinode print something like:

```
Thu 2/15 15:01 prove_fib: begin
Thu 2/15 15:01 a: 2081405077
Thu 2/15 15:01 b: 315178285
Thu 2/15 15:01 succesfully generated and verified proof for the program!
```
