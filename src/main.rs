extern crate num;
extern crate mod_pow;
extern crate modexp;
extern crate ramp;
extern crate rand;

use rand::{Rng};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{stdin,stdout,Write};

use ramp::Int;

fn main() {
    let print_num = 1000000;
    let ramp_base;
    let mut i: i64;
    let ramp_mod;
    let ramp_goal;

    println!("This will calculate x for (base ^ x mod modulus = remainder goal)");
    if get_user_input("Use random numbers? (Y/n) ") == "Y" {
        let mut rng = rand::thread_rng();
        i = 0;
        let rnd_min = 100000000;
        let rnd_max = 999999999;
        ramp_base = Int::from(rng.gen_range(rnd_min, rnd_max));
        let ramp_exponent = Int::from(rng.gen_range(rnd_min, rnd_max));
        ramp_mod = Int::from(rng.gen_range(rnd_min, rnd_max));
        ramp_goal = ramp_base.pow_mod(&ramp_exponent, &ramp_mod);

        println!("Base is {}", ramp_base);
        println!("Exponent is {}", ramp_exponent);
        println!("Mod is {}", ramp_mod);
        println!("Goal is {}", ramp_goal);
    }
    else {
        ramp_base = Int::from_str(&get_user_input("What is the base? ")).unwrap();
        i = get_user_input("What should the exponent count start at? (Normally 0) ").parse().unwrap();
        ramp_mod = Int::from_str(&get_user_input("What is the modulus? ")).unwrap();
        ramp_goal = Int::from_str(&get_user_input("What is the remainder goal? ")).unwrap();
    }

    let mut ramp_mod_exp = Int::from_str("0").unwrap();
    let mut last_time = get_time();

    while ramp_mod_exp != ramp_goal {
        i += 1;
        let ramp_exponent_count = Int::from(i);

        ramp_mod_exp = ramp_base.pow_mod(&ramp_exponent_count, &ramp_mod);
        
        if i % print_num == 0 {
            let elapsed_time = get_time() - last_time;
            last_time = get_time();
            println!("{}ms - Exponent is at {}", elapsed_time, i);
        }
    }
    println!("Found x - {} ^ {} mod {} = {}", ramp_base, i, ramp_mod, ramp_mod_exp);
}

fn get_time() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    return since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
}

fn get_user_input(question: &str) -> String {
    let mut s = String::new();
    print!("{}", question);
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    return s;
}