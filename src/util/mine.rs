

pub  const DIFFICULTY:u32=1;
pub fn is_hash_valid(hash: &str, difficulty: usize) -> bool {
    let prefix = "0".repeat(difficulty);
    hash.starts_with(&prefix)
}