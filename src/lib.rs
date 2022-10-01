mod utils;

pub fn get_hash(flag: u8, data: &str) -> String {
    let mut message: Vec<u32> = Vec::new();
    if flag == 102_u8 {
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

    let parsed_message: Vec<Vec<u32>> = utils::msg::get_parsed_message(message);

    utils::scheduler::hash(parsed_message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_string() {
        let flag: u8 = 115;
        let data: &str = "This is a test string.";

        assert_eq!(get_hash(flag, data), "3eec256a587cccf72f71d2342b6dfab0bbca01697c7e7014540bdd62b72120da");
    }

    #[test]
    fn test_file() {
        let flag: u8 = 102;
        let mut path: String = get_current_dir();
        path.push_str("/src/utils/msg.rs");
        let data: &str = &path[..];
        println!("{}", data);

        assert_eq!(get_hash(flag, data), "580ad3ed9192cdbe47ec2d447dc63cd6b78ac8fc45997b5b17b13c44786d2511");
    }

    fn get_current_dir() -> String {
        let res = env::current_dir();
        match res {
            Ok(path) => path.into_os_string().into_string().unwrap(),
            Err(_) => "FAILED".to_string(),
        }
    }
}

