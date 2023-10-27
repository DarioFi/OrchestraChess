fn respond_to_uci(message: &str) {
    println!("{}", message);
}


pub fn split_fen_moves(s: &str) -> (String, String) {
    if !s.contains("moves") {
        return (s.replace("fen ", ""), String::from(""));
    }

    let parts: Vec<&str> = s.split("moves").collect();

    let fen = parts[0].replace("fen ", "");
    let moves = String::from(parts[1]);

    (fen, moves)
}

#[inline]
pub fn lsb(x: u64) -> u8 {
    x.trailing_zeros() as u8
}
#[inline]

pub fn remove_lsb(x: u64) -> u64 {
    if x == 0{
        return 0_u64
    }
    x & (x - 1)
}

pub fn get_lsb_masked(x: u64) -> u64 {
    // this function gets the lsb and creates a mask with all 0 except that one
    // e.g. 0000_0000_0000_0000_0000_0000_0000_0000_0000_0001_0000_0000_0000_0000_0000_0000
    // becomes 0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001_0000_0000_0000_0000_0000
    x & (x.wrapping_neg())
}

pub fn get_msb_index(x: u64) -> u8 {
    63 - x.leading_zeros() as u8
}

pub fn get_msb_masked(x: u64) -> u64 {
    1 << get_msb_index(x)
}

pub fn remove_msb(x: u64) -> u64 {
    x ^ get_msb_masked(x)
}

pub fn pop_count(x: u64) -> u32 {
    x.count_ones()
}