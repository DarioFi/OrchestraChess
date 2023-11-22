// use std::fs::File;
// use std::io::{self, Read};
//
// const LEB128_MAGIC_STRING: &[u8; 4] = b"LEB1";
// const LEB128_MAGIC_STRING_SIZE: usize = 4;
//
// fn read_leb_128<T: Copy + Default + std::fmt::Debug>(stream: &mut File, out: &mut [T], count: usize) -> io::Result<()> {
//     // Check the presence of our LEB128 magic string
//     let mut leb128_magic_string = [0_u8; LEB128_MAGIC_STRING_SIZE];
//     stream.read_exact(&mut leb128_magic_string)?;
//     assert_eq!(&leb128_magic_string, LEB128_MAGIC_STRING);
//
//     // Ensure the type is signed (not implemented for unsigned types)
//
//     const BUF_SIZE: usize = 4096;
//     let mut buf = [0_u8; BUF_SIZE];
//
//     let mut bytes_left = read_little_endian::<u32>(stream)?;
//
//     let mut buf_pos = BUF_SIZE;
//     for i in 0..count {
//         let mut result: T = Default::default();
//         let mut shift = 0;
//
//         while shift < std::mem::size_of::<T>() * 8 {
//             if buf_pos == BUF_SIZE {
//                 let bytes_to_read = std::cmp::min(bytes_left, BUF_SIZE as u32);
//                 stream.read_exact(&mut buf[0..bytes_to_read as usize])?;
//                 buf_pos = 0;
//             }
//
//             let byte = buf[buf_pos];
//             buf_pos += 1;
//             bytes_left -= 1;
//
//             result = result | ((byte & 0x7f) as T) << shift;
//             shift += 7;
//
//             if (byte & 0x80) == 0 {
//                 out[i] = if std::mem::size_of::<T>() * 8 <= shift || (byte & 0x40) == 0 {
//                     result
//                 } else {
//                     result | !((1 << shift) - 1)
//                 };
//                 break;
//             }
//         }
//     }
//
//     assert_eq!(bytes_left, 0);
//
//     Ok(())
// }
//
// fn read_little_endian<T: Read>(stream: &mut File) -> io::Result<T> {
//     let mut buffer = [0_u8; std::mem::size_of::<T>()];
//     stream.read_exact(&mut buffer)?;
//     Ok(T::from_le_bytes(buffer))
// }
//
//
