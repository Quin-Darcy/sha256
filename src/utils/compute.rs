#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

//use std::vec::Vec;

pub mod sha {
    pub fn sigma_0_256(x: u32) -> u32 {
        let rotr_7: u32 = (x >> 7) | (x << 32-7);
        let rotr_18: u32 = (x >> 18) | (x << 32-18);
        let shr_3: u32 = x >> 3;
    
        rotr_7 ^ rotr_18 ^ shr_3
    }
    
    pub fn sigma_1_256(x: u32) -> u32 { // 17 19 10
        let rotr_17: u32 = (x >> 17) | (x << 32-17);
        let rotr_19: u32 = (x >> 19) | (x << 32-19);
        let shr_10: u32 = x >> 10;
    
        rotr_17 ^ rotr_19 ^ shr_10
    }
    
    pub fn SIGMA_0_256(x: &u32) -> u32 {
        let rotr_2: u32 = (x >> 2) | (x << 32-2);
        let rotr_13: u32 = (x >> 13) | (x << 32-13);
        let rotr_22: u32 = (x >> 22) | (x << 32-22);
    
        rotr_2 ^ rotr_13 ^ rotr_22
    }
    
    pub fn SIGMA_1_256(x: &u32) -> u32 { //6 11 25
        let rotr_6: u32 = (x >> 6) | (x << 32-6);
        let rotr_11: u32 = (x >> 11) | (x << 32-11);
        let rotr_25: u32 = (x >> 25) | (x << 32-25);
    
        rotr_6 ^ rotr_11 ^ rotr_25
    }
    
    pub fn ch(x: &u32, y: &u32, z: &u32) -> u32 {
        (x & y) ^ (!x & z)
    }
    
    pub fn maj(x: &u32, y: &u32, z: &u32) -> u32 {
        (x & y) ^ (x & z) ^ (y & z)
    }
    
    pub fn sha_constants(n: u32, m: u32) -> u32 {
        let root: f64 = (n as f64).powf(1.0/(m as f64));
        let fractional_part: f64 = (root - root.floor()) * ((16_u64).pow(8_u32) as f64);
        fractional_part as u32
    }
}

pub mod converter {
    pub fn bin_to_num(bin_rep:  Vec<u32>) -> u32 {
        let num_of_bits: usize = bin_rep.len();
        let mut dec_rep: u32 = 0;
    
        for i in 0..num_of_bits {
            dec_rep += bin_rep[num_of_bits-1-i] * 2_u32.pow(i as u32); 
        }
        dec_rep
    }
    
    pub fn num_to_bin(num: u32, num_of_bits: u32) -> Vec<u32> {
        let base: u32 = 2;
        let mut ex: u32 = 0;
        let mut num: u32 = num.clone();
        let mut bin_rep: Vec<u32> = vec![0; num_of_bits as usize];
    
        while num > 0 {
            if base.pow(ex) <= num {
                ex = ex + 1;
            } else {
                num = num - base.pow(ex-1);
                bin_rep[(num_of_bits-ex) as usize] = 1;
                ex = 0;
            }
        }
        bin_rep
    }
}