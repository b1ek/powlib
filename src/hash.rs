use scrypt::{scrypt, Params};

use crate::num::Num;

const SALT: [u8; 0] = [];

pub fn hash(data: &[u8], out: &mut [u8]) {
    let params = Params::new(6, 64, 1, 32).unwrap();
    scrypt(data, &SALT.clone(), &params, out).unwrap();
}

pub fn hash_num(num: Num) -> [u8; 32] {
    let bnum = num.bytes();
    let mut out: [u8; 32] = [0; 32];
    hash(&bnum, &mut out);
    out
}