#![no_std]
#![no_main]

use core::usize;

#[macro_use]
extern crate user_lib;

const LEN: usize = 100;

#[unsafe(no_mangle)]
fn main() -> i32 {
    let p = 5u64;
    // 模数：常用的大质数，用于避免数值溢出
    let m = 998244353u64;
    let iter: usize = 140000;
    let mut s = [0u64; LEN];
    let mut cur = 0usize;
    s[cur] = 1;
    for i in 1..=iter {
        let next = if cur + 1 == LEN { 0 } else { cur + 1 };
        s[next] = s[cur] * p % m;
        cur = next;
        if i % 10000 == 0 {
            println!("power_5 [{}/{}]", i, iter);
        }
    }
    println!("{}^{} = {}(MOD {})", p, iter, s[cur], m);
    println!("Test power_5 OK!");
    0
}
