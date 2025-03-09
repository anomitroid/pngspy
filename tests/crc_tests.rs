#[cfg(test)]
mod tests {
    use pngspy::utils::crc::crc32;

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
