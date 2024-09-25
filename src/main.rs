use alloy_core::primitives::{Address, FixedBytes, U256};
use std::thread;
use std::time::Instant;
use alloy_core::hex;
use tiny_keccak::{Hasher, Keccak};

fn increment_fixed_bytes(val: &mut FixedBytes<32>) {
    let bytes: &mut [u8] = val.as_mut_slice();

    for byte in bytes.iter_mut().rev() {
        if *byte < 255 {
            *byte += 1;
            break;
        } else {
            *byte = 0;
        }
    }
}

fn find_matching_address(sender: Address, deployer: Address, init_code_hash: FixedBytes<32>, pattern: Address, mask: Address, start_salt: FixedBytes<32>, end_salt: FixedBytes<32>) {
    let mut salt_seed = start_salt;
    let start = Instant::now();

    loop {
        // Convert sender address to bytes32
        let sender_bytes32 = {
            let mut bytes = [0u8; 32];
            bytes[12..32].copy_from_slice(&sender.0[..]); // Ethereum addresses are 20 bytes, so pad with leading zeros
            bytes
        };

        // Convert sender.0 (FixedBytes<20>) to &[u8] using as_ref()
        let sender_bytes20: &[u8] = sender.0.as_ref();

        // Create a buffer to fit sender_bytes20 (20 bytes) and salt_seed (12 bytes) for a total of 32 bytes
        let mut combined: Vec<u8> = Vec::with_capacity(32);
        combined.extend_from_slice(sender_bytes20); // Use the byte slice directly (20 bytes)
        combined.extend_from_slice(&salt_seed.as_ref()); // Add the salt_seed (12 bytes)

        // Concatenate sender_bytes32 and combined (32 bytes each)
        let mut to_hash: Vec<u8> = Vec::with_capacity(64);
        to_hash.extend_from_slice(&sender_bytes32); // 32 bytes
        to_hash.extend_from_slice(&combined);     

        // Calculate the Keccak256 hash
        let mut keccak = Keccak::v256();
        let mut salt = [0u8; 32];
        keccak.update(&to_hash);
        keccak.finalize(&mut salt);


        let address: Address = deployer.create2(salt, init_code_hash);

        if address.bit_and(mask).bit_xor(pattern).is_zero() {
            println!("Found matching address: {:?}, for salt: {:?}", address, hex::encode(combined));
            println!("Time elapsed is: {:?}", start.elapsed());
            //break;
            std::process::exit(1);
        }
        if salt_seed == end_salt {
            break;
        }
        increment_fixed_bytes(&mut salt_seed);
    }
}

/// CreateX.deployCreate3()
//  salt = keccak256[
//                       bytes32(uint256(uint160(address(sender)))) 
//                     + bytes32(bytes20(address(sender)) + bytes12(salt_seed))
//                 ]

fn main() {
    let sender = Address::from_slice(&hex::decode("92Cd849801A467098cDA7CD36756fbFE8A30A036").unwrap());
    let deployer = Address::from_slice(&hex::decode("ba5Ed099633D3B313e4D5F7bdc1305d3c28ba5Ed").unwrap()); // createX address https://createx.rocks/
    let init_code_hash = FixedBytes::from_slice(&hex::decode("21c35dbe1b344a2488cf3321d6ce542f8e9f305544ff09e4993a62319a497c1f").unwrap()); // minimal proxy initcode https://github.com/pcaversaccio/createx/blob/472ca357d00d1ad340f118c8d446b0ece818f465/src/CreateX.sol#L632C47-L632C94
    let pattern = Address::from_slice(&hex::decode("ffff000000000000000000000000000000000000").unwrap());
    let mask = Address::from_slice(&hex::decode("ffff000000000000000000000000000000000000").unwrap());

    let num_threads = 16; // Number of threads

    let salt_range = U256::from(79228162514264337593543950335_i128) / U256::from(num_threads);

    let mut threads = vec![];
    for i in 0..num_threads {
        let deployer = deployer;
        let init_code_hash = init_code_hash;
        let pattern = pattern;
        let mask = mask;

        let start_salt = FixedBytes::from(U256::from(i) * salt_range);
        let end_salt = FixedBytes::from(U256::from(i + 1) * salt_range);
        
        let thread = thread::spawn(move || {
            find_matching_address(sender, deployer, init_code_hash, pattern, mask, start_salt, end_salt);
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }
}