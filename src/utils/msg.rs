use std::io::Read;
use std::fs::File;
use std::io::BufReader;
use crate::utils::compute::converter;


const BLOCKSIZE: usize = 512;
const WORD_LENGTH: usize = 32;
const WORDS_IN_BLOCK: usize = BLOCKSIZE / WORD_LENGTH;


// Opens file located at PATH and returns vector of bytes read from the file.
// In addition to bytes read from file, the vector is padded with 0s such that
// the length of the vector is 64 less than a multiple of BLOCKSIZE. NOTE: the 
// first padded bit is '1'.
pub fn get_binary(file_path: &str) -> Result<Vec<u32>, std::io::Error> {
    let mut message_bytes: Vec<u8> = Vec::new();
    let mut message: Vec<u32> = Vec::new();
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut reader = BufReader::new(file);
    
    let bits_in_message: usize = match reader.read_to_end(&mut message_bytes) {
        Ok(n) => 8*n,
        Err(e) => return Err(e),
    };
   
    for byte in message_bytes {
        message.extend(converter::num_to_bin(byte as u32, 8));
    }
    
    let message_padding: Vec<u32> = get_padding(bits_in_message as u32); 

    message.extend(message_padding);
    
    Ok(message)
}

// Given num_of_bits in message, this function calculates the nearest multiple
// of BLOCKSIZE which is greater than num_of_bits and returns the a vector 
// containing a pad starting with a 1 followed by n number of zeros and ending
// with 64 bits containing num_of_bits in binary.
pub fn get_padding(num_of_bits: u32) -> Vec<u32> {
    let mut bits_in_padding: u32 = 0;
    let mut pad: Vec<u32> = Vec::new();
    let blocksize: u32 = BLOCKSIZE.clone() as u32;
    
    while (num_of_bits+1+bits_in_padding) % blocksize != 448 {
        bits_in_padding += 1;
    }

    let difference: Vec<u32> = vec![0; bits_in_padding as usize];
    let message_len_as_bin: Vec<u32> = converter::num_to_bin(num_of_bits, 32);
    let empty_block: Vec<u32> = vec![0; 32 as usize];

    pad.push(1);
    pad.extend(difference);
    pad.extend(empty_block);
    pad.extend(message_len_as_bin);
 
    pad
}

// The message and its padding are parsed into N 512-bit blocks.
// A vector containing the N blocks each as 16 32-bit words is returned. 
pub fn get_parsed_message(message: &Vec<u32>) -> Vec<Vec<u32>> {
    let bits_in_message: usize = message.len();
    let mut parsed_message: Vec<Vec<u32>> = Vec::new();
    let blocks_in_message: usize = bits_in_message / BLOCKSIZE;

    for i in 0..blocks_in_message {
        let mut words: Vec<u32> = Vec::new();
        let block: Vec<u32> = message[i*BLOCKSIZE..(i+1)*BLOCKSIZE].to_vec();
        for j in 0..WORDS_IN_BLOCK {
            words.push(converter::bin_to_num(block[j*WORD_LENGTH..(j+1)*WORD_LENGTH].to_vec()));
        }
        parsed_message.push(words);
    }
    parsed_message
}
