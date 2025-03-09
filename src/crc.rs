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