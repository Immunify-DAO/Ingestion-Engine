use std::{fs::File, io::Read};
use anyhow::{Result, bail};
use sha2::{Sha256, Digest};

pub fn hash_file(file_name: &str) -> Result<[u8; 32]> {
    let mut hasher = Sha256::new();
    let mut file = File::open(file_name)?;
    let mut file_data = Vec::new();

    file.read_to_end(&mut file_data)?;
    hasher.update(&file_data);

    let hash = hasher.finalize();
    let mut ret: [u8; 32] = <[u8; 32]>::default();

    ret.copy_from_slice(&hash);

    Ok(ret)
}

pub fn hex_to_string(data: &[u8]) -> String {
    let mut ret = String::new();

    for d in data {
        let x = format!("{:02x}", d);
        ret.push_str(&x);
    }

    ret
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_sum() {
        let hash = hash_file("test_data/test1.bin").expect("didnt hash properly");
        let hash_string = hex_to_string(&hash);
        let expected_result = "f0887fe961c9cd3beab957e8222494abb969b1ce4c6557976df8b0f6d20e9166"; 
        assert_eq!(&hash_string, expected_result);

    }


}
