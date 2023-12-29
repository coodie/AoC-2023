
pub fn pretty_print_bytes(bytes: &[u8]) {
    for byte in bytes {
        print!("{}", *byte as char);
    }
    println!();
}

pub fn pretty_print_byte_list(byte_list: &[Vec<u8>]) {
    for sub_list in byte_list {
        pretty_print_bytes(sub_list);
    }
    println!();
}
