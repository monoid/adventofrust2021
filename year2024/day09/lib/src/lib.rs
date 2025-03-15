pub type Id = u64;
pub type Count = usize;

pub fn range_checksum(start: usize, len: usize, id: Id) -> u64 {
    let sum = (start + start + len - 1) * len / 2;
    id * (sum as u64)
}
