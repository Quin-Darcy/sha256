pub mod utils;


fn get_digest(data: &[u8]) -> Vec<u32> {
    let mut message: Vec<u32> = Vec::new();
    let bytes: Vec<u8> = data.to_vec();

    for byte in bytes {
        message.extend(utils::compute::converter::num_to_bin(byte as u32, 8));
    }
    message.extend(utils::msg::get_padding(message.len() as u32)); 
    
    let parsed_message: Vec<Vec<u32>> = utils::msg::get_parsed_message(&message);

    utils::scheduler::digest(parsed_message)
}

pub fn get_hash(data: &[u8]) -> Vec<u8> {
    let digest: Vec<u32> = get_digest(data);
    let mut bytes: [u8; 32] = [0; 32];

    let mut i: usize = 0;
    for block in digest {
        let temp_bytes = block.to_be_bytes();
        for byte in temp_bytes {
            bytes[i] = byte;
            i += 1;
        }
    }
    bytes.to_vec()
}