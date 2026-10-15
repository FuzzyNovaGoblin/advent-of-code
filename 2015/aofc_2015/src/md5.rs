pub fn md5_hash(mut input: Vec<u8>) {
    let needed_bytes = (512 - (512 % ((input.len() * 8) + 64)))/8;
    input.append(&mut vec![0; needed_bytes]);
}
