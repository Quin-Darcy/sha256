#![allow(non_snake_case)]
use crate::utils::compute::sha;


const BLOCKSIZE: usize = 512;
const WORD_LENGTH: usize = 32;
const WORDS_IN_BLOCK: usize = BLOCKSIZE / WORD_LENGTH;
const WORDS_IN_SCHEDULE: usize =  4 * WORDS_IN_BLOCK;
const FIRST_PRIMES: [u32; 64] = [2, 3, 5, 7, 11, 13, 17, 19, 
                                 23, 29, 31, 37, 41, 43, 47, 53, 
                                 59, 61, 67, 71, 73, 79, 83, 89, 
                                 97, 101, 103, 107, 109, 113, 127, 131, 
                                 137, 139, 149, 151, 157, 163, 167, 173, 
                                 179, 181, 191, 193, 197, 199, 211, 223, 
                                 227, 229, 233, 239, 241, 251, 257, 263, 
                                 269, 271, 277, 281, 283, 293, 307, 311];


pub fn digest(parsed_message: Vec<Vec<u32>>) -> Vec<u32> {
    let blocks_in_message: usize = parsed_message.len();
    let mut message_schedule = [0_u32; WORDS_IN_SCHEDULE];
    let mut H: Vec<u32> = Vec::new();

    for i in 0..8 {
        H.push(sha::sha_constants(FIRST_PRIMES[i], 2));
    }

    for i in 1..blocks_in_message + 1 {
        
        for t in 0..WORDS_IN_SCHEDULE {
            if t < WORDS_IN_BLOCK {
                message_schedule[t] = parsed_message[i-1][t];
            } else {
                let w1: u32 = message_schedule[t-2].clone();
                let w2: u32 = message_schedule[t-7].clone();
                let w3: u32 = message_schedule[t-15].clone();
                let w4: u32 = message_schedule[t-16].clone();

                // Use of wrapping add for expected overflow
                message_schedule[t] = sha::sigma_1_256(w1)
                    .wrapping_add(w2)
                    .wrapping_add(sha::sigma_0_256(w3))
                    .wrapping_add(w4);

            }
        }
        
        let mut a: u32 = H[0].clone(); 
        let mut b: u32 = H[1].clone();
        let mut c: u32 = H[2].clone();
        let mut d: u32 = H[3].clone();
        let mut e: u32 = H[4].clone();
        let mut f: u32 = H[5].clone();
        let mut g: u32 = H[6].clone();
        let mut h: u32 = H[7].clone();
        
        for t in 0..WORDS_IN_SCHEDULE {
            let k: u32 = sha::sha_constants(FIRST_PRIMES[t], 3);
            
            let t1: u32 = h
                .wrapping_add(sha::SIGMA_1_256(&e))
                .wrapping_add(sha::ch(&e, &f, &g))
                .wrapping_add(k)
                .wrapping_add(message_schedule[t]);

            let t2: u32 = sha::SIGMA_0_256(&a)+sha::maj(&a, &b, &c);
            h = g;
            g = f;
            f = e;
            e = d+&t1;
            d = c;
            c = b;
            b = a;
            a = t1+t2;
        }

        H[0] = a + H[0];
        H[1] = b + H[1];
        H[2] = c + H[2];
        H[3] = d + H[3];
        H[4] = e + H[4];
        H[5] = f + H[5];
        H[6] = g + H[6];
        H[7] = h + H[7];
    }
    H
}
