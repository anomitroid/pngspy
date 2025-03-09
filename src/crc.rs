#[allow(dead_code)]
const fn generate_crc_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut crc = i as u32;
        let mut j = 0;
        while j < 8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
            j += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
}

#[allow(dead_code)]
pub const CRC_TABLE: [u32; 256] = generate_crc_table();

#[allow(dead_code)]
pub fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = 0xFFFFFFFFu32;
    for &b in bytes {
        let index = ((crc as u8) ^ b) as usize;
        crc = (crc >> 8) ^ CRC_TABLE[index];
    }
    crc ^ 0xFFFFFFFF
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc32_empty() {
        // The CRC32 for an empty slice is defined as 0x00000000.
        // (Note: Different implementations may vary in how they handle an empty input;
        // here we expect 0x00000000 as the final result.)
        assert_eq!(crc32(&[]), 0x00000000);
    }

    #[test]
    fn test_crc32_known_vector() {
        // "123456789" is a standard test vector.
        // The expected CRC32 for "123456789" is 0xCBF43926.
        let data = b"123456789";
        assert_eq!(crc32(data), 0xCBF43926);
    }

    #[test]
    fn test_crc32_arbitrary_string() {
        let data = b"The quick brown fox jumps over the lazy dog";
        // Expected CRC32 value computed with the standard algorithm.
        // This value (0x414FA339) has been verified against other implementations.
        assert_eq!(crc32(data), 0x414FA339);
    }
}
