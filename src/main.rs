extern crate num;
extern crate mod_pow;

use num::bigint::{ BigInt, BigUint, Sign };
use num::traits::FromPrimitive;
use std::time::{SystemTime, UNIX_EPOCH};

use mod_pow::ModPow;

fn main() {
    let print_num = 100000;
    let mut last_time = get_time();

    fn check(b: &BigInt, e: &BigInt, m: &BigUint) -> BigInt {
        return b.mod_pow(&e, &m);
    }

    let mut i = 1;
    let mut num = BigInt::new(Sign::Plus, vec![0]);
    let base = BigInt::new(Sign::Plus, vec![7]);
    let goal = BigInt::new(Sign::Plus, vec![10,483,580,695,461,280,548,150,531]);
    let mod_num = BigUint::parse_bytes(b"15045919506432000000000001", 10).unwrap();

    while num != goal {
        let exponent = BigInt::from_i64(i).unwrap();
        num = check(&base, &exponent, &mod_num);
        i += 1;
        if i % print_num == 0 {
            let elapsed_time = get_time() - last_time;
            last_time = get_time();
            println!("{}ms - {}", elapsed_time, i);
        }
    }
}

fn get_time() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    return since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
}