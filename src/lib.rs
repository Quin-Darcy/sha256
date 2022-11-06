pub mod utils;
use num_bigint::BigUint;


fn get_digest(flag: &u8, data: &str) -> Vec<u32> {
    let mut message: Vec<u32> = Vec::new();
    if *flag == 102_u8 {
        message = match utils::msg::get_binary(data) {
            Ok(b) => b,
            Err(_e) => panic!("Error. Could not get bytes from {}", data),
        }; 
    } else {
        let bytes: Vec<u8> = data.as_bytes().to_vec();
        for byte in bytes {
            message.extend(utils::compute::converter::num_to_bin(byte as u32, 8));
        }
        message.extend(utils::msg::get_padding(message.len() as u32)); 
    }
    println!("here");
    let parsed_message: Vec<Vec<u32>> = utils::msg::get_parsed_message(&message);

    utils::scheduler::digest(parsed_message)
}

pub fn get_hash(flag: &u8, data: &str) -> BigUint {
    let digest: Vec<u32> = get_digest(flag, data);
    let mut bytes: [u8; 32] = [0; 32];

    let mut i: usize = 0;
    for block in digest {
        let temp_bytes = block.to_be_bytes();
        for byte in temp_bytes {
            bytes[i] = byte;
            i += 1;
        }
    }
    BigUint::from_bytes_be(&bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_string() {
        let flag: u8 = 115;
        let data: &str = "This is a test string.";
        let str_hash: String = "3eec256a587cccf72f71d2342b6dfab0bbca01697c7e7014540bdd62b72120da".to_string(); 

        assert_eq!(format!("{:0x}", get_hash(&flag, data)).to_string(), str_hash);
    }

    #[test]
    fn test_file() {
        let flag: u8 = 102;
        let mut path: String = get_current_dir();
        let str_hash: String;

        if env::consts::OS == "windows" {
            path.push_str("LICENSE");
            str_hash = "8253caed2ff88d53aee87dbf8bf8b88afb16eac3fd524d0e4bbae8e95ccf99c4".to_string();
        } else {
            path.push_str("/LICENSE");
            str_hash = "6022e2824ee92fcce1cdc0bfb71b8ad2428b73f939a780a115605cf47f580755".to_string();
        }

        let data: &str = &path[..];
        assert_eq!(format!("{:0x}", get_hash(&flag, data)).to_string(), str_hash);
    }

    fn get_current_dir() -> String {
        let res = env::current_dir();
        match res {
            Ok(path) => path.into_os_string().into_string().unwrap(),
            Err(_) => "FAILED".to_string(),
        }
    }
}

