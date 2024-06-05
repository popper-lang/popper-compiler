use sha2::Digest;
use std::fs::File;
use std::fmt::Write;

pub fn hash_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut hasher = sha2::Sha256::new();
    std::io::copy(&mut file, &mut hasher).expect("failed to copy file");

    let hash = hasher
        .finalize()
        .to_vec()
        .iter()
        .fold(String::new(), |mut acc, x| {
            write!(acc, "{:02x}", x).unwrap();
            acc
        });
    hash
}
