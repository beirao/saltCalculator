# Create2 salt calculator

This is a Rust program that finds a matching address for a given sender, initialization code hash, pattern, and mask using the Ethereum Create2 address generation algorithm. The program uses multi-threading to speed up the search process.

## Setup

```
git clone https://github.com/beirao/.git
cd create2-address-finder
cargo run
```

The program will search for a matching address and print the result to the console.

## Configuration

The program can be configured by modifying the `main` function in `src/main.rs`. The following parameters can be adjusted:

- `sender`: The address of the sender.
- `init_code_hash`: The initialization code hash.
- `pattern`: The pattern to match against the generated address.
- `mask`: The mask to apply to the generated address.
- `num_threads`: The number of threads to use for the search.

## Dependencies

This project depends on the `alloy-core` crate, which provides the `Address` and `FixedBytes` types used in the program. To add the dependency to your project, add the following line to your `Cargo.toml` file:

```
[dependencies]
alloy-core = "0.2.0"
```

## License

This project is licensed under the MIT License. See the `LICENSE` file for more information.