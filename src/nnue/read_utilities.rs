use std::fs::File;
use std::io::Read;

pub fn read_u32(file: &mut File) -> u32 {
    let mut buffer = [0_u8; 4];
    file.read_exact(&mut buffer).expect("Unable to read file");
    u32::from_le_bytes(buffer)
}

pub fn read_i32(file: &mut File) -> i32 {
    let mut buffer = [0_u8; 4];
    file.read_exact(&mut buffer).expect("Unable to read file");
    i32::from_le_bytes(buffer)
}

pub fn read_i8(file: &mut File) -> i8 {
    let mut buffer = [0_u8; 1];
    file.read_exact(&mut buffer).expect("Unable to read file");
    i8::from_le_bytes(buffer)
}


const SIMD_SIZE: usize = 8;

pub fn get_padded(dims: usize) -> usize {
    let remainder = dims % SIMD_SIZE;
    if remainder == 0 {
        dims
    } else {
        dims + SIMD_SIZE - remainder
    }
}