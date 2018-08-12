//! A helper module for commonly used internal functions

/// Convert bytes into `[u32]` blocks for internal representations.
///
/// # Example
/// ```
/// extern crate yogcrypt;
/// use yogcrypt::basic::helper::bytes_to_u32_blocks;
///
/// let msg = b"abcde";
/// let (msg, bit_len) = bytes_to_u32_blocks(msg);
/// assert_eq!(msg, vec![0x61626364, 0x65000000]);
/// assert_eq!(bit_len, 40);
/// ```
pub fn bytes_to_u32_blocks(msg: &[u8]) -> (Vec<u32>, usize) {
    // bit length = msg.len() * 8
    let bit_len = msg.len() << 3;
    // length for [u32] is ceil(msg.len() / 4)
    let mut msg2: Vec<u32> = vec![];
    for index in 0..((msg.len() + 3) / 4) {
        #[inline(always)]
        fn group_as_u32(msg: &[u8], i: usize) -> u32 {
            #[inline(always)]
            fn unpack(o: Option<&u8>) -> u32 {
                match o {
                    None => 0u32,
                    Some(&a) => u32::from(a),
                }
            }
            let start = i * 4;
            (unpack(msg.get(start)) << 24)
                + (unpack(msg.get(start + 1)) << 16)
                + (unpack(msg.get(start + 2)) << 8)
                + unpack(msg.get(start + 3))
        }
        msg2.push(group_as_u32(msg, index));
    }
    (msg2, bit_len)
}
