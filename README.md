# Rust modular exponentiation calculator

## Description

This is a tool to brute force the exponent needed to satisfy the equation\
`base ^ x % modulus = remainder`

It is useful for CTFs, and factoring different types of encryption. This implementation currently takes around 3 seconds to evaluate a million values.

## Instructions
To run, make sure you have Rust nightly installed. Then, run\
`cargo run --release`\
from the parent directory. When prompted, type "Y" for a demo using random numbers, or anything else to use your own numbers.

## mod_exp.py
This is a Python implementation for speed comparisons.
