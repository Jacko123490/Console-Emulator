use std::fmt::{Binary, UpperHex};

pub fn mem_array_display(array: &[u8]) -> String {
    array.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn mem_array_str(array: &[u8]) -> String {
    std::str::from_utf8(array).unwrap().to_string()
}

pub fn mem_entry_display<T: UpperHex>(entry: T) -> String {
    format!("{:02X}", entry)
}

pub fn mem_entry_bits<T: Binary>(entry: T) -> String {
    format!("{:#010b}", entry)
}
