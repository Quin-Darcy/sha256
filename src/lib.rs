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

