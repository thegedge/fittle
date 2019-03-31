use std::ops::{
  ShlAssign,
  BitOrAssign,
};

use num_traits::identities::Zero;

pub struct BitReader<'a> {
    // (position with data, bit within current T)
    cursor: (usize, usize),
    data: &'a [u8],
}

impl<'a> BitReader<'a> {
    pub fn new(data: &'a [u8]) -> BitReader<'a> {
        BitReader {
            cursor: (0, 0),
            data: data,
        }
    }

    pub fn read<V>(&mut self, mut nbits: usize) -> Option<V>
        where
            V: Zero + ShlAssign<usize> + BitOrAssign<V> + From<u8> + std::fmt::Debug + std::fmt::Binary
    {
        let remaining_bits = 8 * (self.data.len() - self.cursor.0) - self.cursor.1;
        if nbits > remaining_bits {
            return None;
        }

        // Special case for u8. Without this, the shift below would otherwise overflow and panic.
        if self.cursor.1 == 0 && nbits == 8 {
            self.cursor.0 += 1;
            return Some(V::from(self.data[self.cursor.0 - 1]));
        }

        let mut result = V::zero();
        while nbits > 0 {
            let bits_remaining = 8 - self.cursor.1;
            if nbits >= bits_remaining {
                let mask = std::u8::MAX << self.cursor.1;
                result <<= bits_remaining;
                result |= V::from((self.data[self.cursor.0] & mask) >> self.cursor.1);

                self.cursor.0 += 1;
                self.cursor.1 = 0;
                nbits -= bits_remaining;
            } else {
                let mask = (std::u8::MAX >> (8 - nbits)) << self.cursor.1;
                result <<= nbits;
                result |= V::from((self.data[self.cursor.0] & mask) >> self.cursor.1);

                self.cursor.1 += nbits;
                nbits = 0;
            }
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_values_aligned_to_byte_boundaries() {
        let data = 0b01101010_11010100_10101001_11111110u32.to_le_bytes();
        let mut reader = BitReader::new(&data);
        assert_eq!(Some(0b11111110), reader.read::<u8>(8));
        assert_eq!(Some(0b10101001_11010100), reader.read::<u16>(16));
        assert_eq!(Some(0b01101010), reader.read::<u8>(8));
    }

    #[test]
    fn should_read_bits_not_aligned_with_byte_boundaries() {
        let data = 0b11010100_10101001_11111110u32.to_le_bytes();
        let mut reader = BitReader::new(&data);
        assert_eq!(Some(0b1111_11101001), reader.read::<u16>(12));
        assert_eq!(Some(0b1010_11010100), reader.read::<u16>(12));
    }

    #[test]
    fn should_return_none_when_all_bits_read() {
        let data = 0b11010100_10101001u16.to_le_bytes();
        let mut reader = BitReader::new(&data);
        assert_eq!(Some(0b10101001), reader.read::<u16>(8));
        assert_eq!(Some(0b11010100), reader.read::<u8>(8));
        assert_eq!(None, reader.read::<u16>(12));
        assert_eq!(None, reader.read::<u16>(1));
    }

    #[test]
    fn should_return_none_when_need_more_bits_than_available() {
        let data = 0b11010100_10101001u16.to_le_bytes();
        let mut reader = BitReader::new(&data);
        assert_eq!(Some(0b1010_10010100), reader.read::<u16>(12));
        assert_eq!(None, reader.read::<u16>(12));
    }

    #[test]
    fn should_return_remaining_bits_after_attempting_to_ready_more_bits_than_available() {
        let data = 0b11010100_10101001u16.to_le_bytes();
        let mut reader = BitReader::new(&data);
        assert_eq!(Some(0b1010_10010100), reader.read::<u16>(12));
        assert_eq!(None, reader.read::<u16>(12));
        assert_eq!(Some(0b1101), reader.read::<u16>(4));
    }
}
