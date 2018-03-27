extern crate num;
extern crate mod_pow;
extern crate modexp;
extern crate ramp;

use num::bigint::{ BigInt, BigUint, Sign };
use num::traits::FromPrimitive;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use mod_pow::ModPow;
use modexp::modexp;
use ramp::Int;

fn main() {
    let print_num = 1000000;
    let mut last_time = get_time();

    let mut i = 1;
    let goal = BigInt::new(Sign::Plus, vec![10,483,580,695,461,280,548,150,531]);
    let mod_numU = BigUint::parse_bytes(b"15045919506432000000000001", 10).unwrap();
    let mod_num = BigInt::new(Sign::Plus, vec![15,045,919,506,432,000,000,000,001]);
    let mut num = BigInt::new(Sign::Plus, vec![0]);
    let base = BigInt::new(Sign::Plus, vec![7]);

    let ramp_goal = Int::from_str("10483580695461280548150531").unwrap();
    let ramp_mod = Int::from_str("15045919506432000000000001").unwrap();
    let ramp_base = Int::from_str("7").unwrap();
    let ramp_num = Int::from_str("0").unwrap();
    let mut ramp_mod_exp = Int::from_str("0").unwrap();;

    while ramp_num != ramp_goal {
        //let exponent = BigInt::from_i64(i).unwrap();
        let ramp_exponent = Int::from(i);

        //let use_mod_pow = base.mod_pow(&exponent, &mod_numU);
        //let use_mod_exp = modexp(base, exponent, mod_num);
        ramp_mod_exp = ramp_base.pow_mod(&ramp_exponent, &ramp_mod);
        
        i += 1;
        if i % print_num == 0 {
            let elapsed_time = get_time() - last_time;
            last_time = get_time();
            println!("{}ms - {} {}", elapsed_time, i, ramp_mod_exp);
        }
    }
    println!("WINNER {} at {}", ramp_mod_exp, i);
}

fn get_time() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    return since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
}