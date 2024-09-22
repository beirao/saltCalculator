use alloy_core::primitives::{Address, FixedBytes, U256};
use std::thread;
use std::time::Instant;

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

fn find_matching_address(sender: Address, init_code_hash: FixedBytes<32>, pattern: Address, mask: Address, start_salt: FixedBytes<32>, end_salt: FixedBytes<32>) {
    let mut salt = start_salt;
    let start = Instant::now();

    loop {
        let address: Address = sender.create2(salt, init_code_hash);

        if address.bit_and(mask).bit_xor(pattern).is_zero() {
            println!("Found matching address: {:?}, for salt: {:?}", address, salt);
            println!("Time elapsed is: {:?}", start.elapsed());
            //break;
            std::process::exit(1);
        }
        if salt == end_salt {
            break;
        }
        increment_fixed_bytes(&mut salt);
    }
}

fn main() {
    let sender = Address::from_slice(&[0x12; 20]);
    let init_code_hash = FixedBytes::from_slice(&[0x34; 32]);
    let pattern = Address::from_slice(&[0xC0, 0xd3, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    let mask = Address::from_slice(&[0xFF, 0xFF, 0xFF, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

    let num_threads = 16; // Number of threads to create

    let salt_range = U256::MAX / U256::from(num_threads);

    let mut threads = vec![];
    for i in 0..num_threads {
        let sender = sender;
        let init_code_hash = init_code_hash;
        let pattern = pattern;
        let mask = mask;

        let start_salt = FixedBytes::from(U256::from(i) * salt_range);
        let end_salt = FixedBytes::from(U256::from(i + 1) * salt_range);

        let thread = thread::spawn(move || {
            find_matching_address(sender, init_code_hash, pattern, mask, start_salt, end_salt);
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
