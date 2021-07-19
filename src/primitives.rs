// LNP/BP client-side-validation foundation libraries implementing LNPBP
// specifications & standards (LNPBP-4, 7, 8, 9, 42, 81)
//
// Written in 2019-2021 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the Apache 2.0 License along with this
// software. If not, see <https://opensource.org/licenses/Apache-2.0>.

//! Taking implementation of little-endian integer encoding

use core::time::Duration;
use std::io;

use super::{Error, StrictDecode, StrictEncode};

impl StrictEncode for bool {
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        (*self as u8).strict_encode(&mut e)
    }
}

impl StrictDecode for bool {
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        match u8::strict_decode(&mut d)? {
            0 => Ok(false),
            1 => Ok(true),
            v => Err(Error::ValueOutOfRange("boolean", 0..1, v as u128)),
        }
    }
}

impl StrictEncode for u8 {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        e.write_all(&[*self][..])?;
        Ok(1)
    }
}

impl StrictDecode for u8 {
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut ret = [0u8; 1];
        d.read_exact(&mut ret)?;
        Ok(ret[0])
    }
}

impl StrictEncode for i8 {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        e.write_all(&self.to_le_bytes())?;
        Ok(1)
    }
}

impl StrictDecode for i8 {
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut ret = [0u8; 1];
        d.read_exact(&mut ret)?;
        Ok(ret[0] as i8)
    }
}

impl StrictEncode for u16 {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        e.write_all(&self.to_le_bytes())?;
        Ok(2)
    }
}

impl StrictDecode for u16 {
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut ret = [0u8; 2];
        d.read_exact(&mut ret)?;
        Ok(u16::from_le_bytes(ret))
    }
}

impl StrictEncode for i16 {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        e.write_all(&self.to_le_bytes())?;
        Ok(2)
    }
}

impl StrictDecode for i16 {
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut ret = [0u8; 2];
        d.read_exact(&mut ret)?;
        Ok(i16::from_le_bytes(ret))
    }
}

impl StrictEncode for u32 {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        e.write_all(&self.to_le_bytes())?;
        Ok(4)
    }
}

impl StrictDecode for u32 {
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut ret = [0u8; 4];
        d.read_exact(&mut ret)?;
        Ok(u32::from_le_bytes(ret))
    }
}

impl StrictEncode for i32 {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        e.write_all(&self.to_le_bytes())?;
        Ok(4)
    }
}

impl StrictDecode for i32 {
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut ret = [0u8; 4];
        d.read_exact(&mut ret)?;
        Ok(i32::from_le_bytes(ret))
    }
}

impl StrictEncode for u64 {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        e.write_all(&self.to_le_bytes())?;
        Ok(8)
    }
}

impl StrictDecode for u64 {
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut ret = [0u8; 8];
        d.read_exact(&mut ret)?;
        Ok(u64::from_le_bytes(ret))
    }
}

impl StrictEncode for i64 {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        e.write_all(&self.to_le_bytes())?;
        Ok(8)
    }
}

impl StrictDecode for i64 {
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut ret = [0u8; 8];
        d.read_exact(&mut ret)?;
        Ok(i64::from_le_bytes(ret))
    }
}

impl StrictEncode for u128 {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        e.write_all(&self.to_le_bytes())?;
        Ok(16)
    }
}

impl StrictDecode for u128 {
    #[inline]
    fn strict_decode<D: io::Read>(d: D) -> Result<Self, Error> {
        Ok(u128::from_le_bytes(<[u8; 16]>::strict_decode(d)?))
    }
}

impl StrictEncode for i128 {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        e.write_all(&self.to_le_bytes())?;
        Ok(16)
    }
}

impl StrictDecode for i128 {
    #[inline]
    fn strict_decode<D: io::Read>(d: D) -> Result<Self, Error> {
        Ok(i128::from_le_bytes(<[u8; 16]>::strict_decode(d)?))
    }
}

impl StrictEncode for usize {
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        if *self > u16::MAX as usize {
            return Err(Error::ExceedMaxItems(*self));
        }
        let size = *self as u16;
        size.strict_encode(&mut e)
    }
}

impl StrictDecode for usize {
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        u16::strict_decode(&mut d).map(|val| val as usize)
    }
}

impl StrictEncode for f32 {
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        e.write_all(&self.to_le_bytes())?;
        Ok(4)
    }
}

impl StrictDecode for f32 {
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut buf: [u8; 4] = [0; 4];
        d.read_exact(&mut buf)?;
        Ok(Self::from_le_bytes(buf))
    }
}

impl StrictEncode for f64 {
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        e.write_all(&self.to_le_bytes())?;
        Ok(8)
    }
}

impl StrictDecode for f64 {
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut buf: [u8; 8] = [0; 8];
        d.read_exact(&mut buf)?;
        Ok(Self::from_le_bytes(buf))
    }
}

impl StrictEncode for Duration {
    #[inline]
    fn strict_encode<E: io::Write>(&self, e: E) -> Result<usize, Error> {
        (self.as_secs(), self.subsec_nanos()).strict_encode(e)
    }
}

impl StrictDecode for Duration {
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        Ok(Self::new(
            u64::strict_decode(&mut d)?,
            u32::strict_decode(&mut d)?,
        ))
    }
}

#[cfg(feature = "chrono")]
mod _chrono {
    use super::*;
    use chrono::{DateTime, NaiveDateTime, Utc};

    impl StrictEncode for NaiveDateTime {
        #[inline]
        fn strict_encode<E: io::Write>(&self, e: E) -> Result<usize, Error> {
            self.timestamp().strict_encode(e)
        }
    }

    impl StrictDecode for NaiveDateTime {
        #[inline]
        fn strict_decode<D: io::Read>(d: D) -> Result<Self, Error> {
            Ok(Self::from_timestamp(i64::strict_decode(d)?, 0))
        }
    }

    impl StrictEncode for DateTime<Utc> {
        #[inline]
        fn strict_encode<E: io::Write>(&self, e: E) -> Result<usize, Error> {
            self.naive_utc().strict_encode(e)
        }
    }

    impl StrictDecode for DateTime<Utc> {
        #[inline]
        fn strict_decode<D: io::Read>(d: D) -> Result<Self, Error> {
            let naive = NaiveDateTime::strict_decode(d)?;
            Ok(DateTime::from_utc(naive, Utc))
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::strict_deserialize;
    use crate::test_helpers::test_encoding_roundtrip;
    use chrono::{NaiveDateTime, Utc};

    #[test]
    fn test_u_encoding() {
        test_encoding_roundtrip(&0_u8, [0]).unwrap();
        test_encoding_roundtrip(&1_u8, [1]).unwrap();
        test_encoding_roundtrip(&0x10_u8, [0x10]).unwrap();
        test_encoding_roundtrip(&0xFE_u8, [0xFE]).unwrap();
        test_encoding_roundtrip(&0xFF_u8, [0xFF]).unwrap();
        test_encoding_roundtrip(&54_u16, [54, 0]).unwrap();
        test_encoding_roundtrip(&0x45a6_u16, [0xa6, 0x45]).unwrap();
        test_encoding_roundtrip(&54_usize, [54, 0]).unwrap();
        test_encoding_roundtrip(&0x45a6_usize, [0xa6, 0x45]).unwrap();
        test_encoding_roundtrip(&54_u32, [54, 0, 0, 0]).unwrap();
        test_encoding_roundtrip(&0x45a6_u32, [0xa6, 0x45, 0, 0]).unwrap();
        test_encoding_roundtrip(&0x56fe45a6_u32, [0xa6, 0x45, 0xfe, 0x56])
            .unwrap();
        test_encoding_roundtrip(&54_u64, [54, 0, 0, 0, 0, 0, 0, 0]).unwrap();
        test_encoding_roundtrip(&0x45a6_u64, [0xa6, 0x45, 0, 0, 0, 0, 0, 0])
            .unwrap();
        test_encoding_roundtrip(
            &0x56fe45a6_u64,
            [0xa6, 0x45, 0xfe, 0x56, 0, 0, 0, 0],
        )
        .unwrap();
        test_encoding_roundtrip(
            &0xcafedead56fe45a6_u64,
            [0xa6, 0x45, 0xfe, 0x56, 0xad, 0xde, 0xfe, 0xca],
        )
        .unwrap();
        test_encoding_roundtrip(
            &54_u128,
            [54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        )
        .unwrap();
        test_encoding_roundtrip(
            &0x45a6_u128,
            [0xa6, 0x45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        )
        .unwrap();
        test_encoding_roundtrip(
            &0x56fe45a6_u128,
            [0xa6, 0x45, 0xfe, 0x56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        )
        .unwrap();
        test_encoding_roundtrip(
            &0xcafedead56fe45a6_u128,
            [
                0xa6, 0x45, 0xfe, 0x56, 0xad, 0xde, 0xfe, 0xca, 0, 0, 0, 0, 0,
                0, 0, 0,
            ],
        )
        .unwrap();
        test_encoding_roundtrip(
            &0xbadefeed65671331cafedead56fe45a6_u128,
            [
                0xa6, 0x45, 0xfe, 0x56, 0xad, 0xde, 0xfe, 0xca, 0x31, 0x13,
                0x67, 0x65, 0xed, 0xfe, 0xde, 0xba,
            ],
        )
        .unwrap();
    }

    #[test]
    fn test_i_encoding() {
        test_encoding_roundtrip(&0_i8, [0]).unwrap();
        test_encoding_roundtrip(&1_i8, [1]).unwrap();
        test_encoding_roundtrip(&0x10_i8, [0x10]).unwrap();
        test_encoding_roundtrip(&0x7E_i8, [0x7E]).unwrap();
        test_encoding_roundtrip(&0x7F_i8, [0x7F]).unwrap();
        test_encoding_roundtrip(&-0x7F_i8, [0x81]).unwrap();
        test_encoding_roundtrip(&54_i16, [54, 0]).unwrap();
        test_encoding_roundtrip(&0x45a6_i16, [0xa6, 0x45]).unwrap();
        test_encoding_roundtrip(&54_i32, [54, 0, 0, 0]).unwrap();
        test_encoding_roundtrip(&0x45a6_i32, [0xa6, 0x45, 0, 0]).unwrap();
        test_encoding_roundtrip(&0x56fe45a6_i32, [0xa6, 0x45, 0xfe, 0x56])
            .unwrap();
        test_encoding_roundtrip(&54_i64, [54, 0, 0, 0, 0, 0, 0, 0]).unwrap();
        test_encoding_roundtrip(&0x45a6_i64, [0xa6, 0x45, 0, 0, 0, 0, 0, 0])
            .unwrap();
        test_encoding_roundtrip(
            &0x56fe45a6_i64,
            [0xa6, 0x45, 0xfe, 0x56, 0, 0, 0, 0],
        )
        .unwrap();
        test_encoding_roundtrip(
            &0x7afedead56fe45a6_i64,
            [0xa6, 0x45, 0xfe, 0x56, 0xad, 0xde, 0xfe, 0x7a],
        )
        .unwrap();
        test_encoding_roundtrip(
            &54_i128,
            [54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        )
        .unwrap();
        test_encoding_roundtrip(
            &0x45a6_i128,
            [0xa6, 0x45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        )
        .unwrap();
        test_encoding_roundtrip(
            &0x56fe45a6_i128,
            [0xa6, 0x45, 0xfe, 0x56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        )
        .unwrap();
        test_encoding_roundtrip(
            &0xcafedead56fe45a6_i128,
            [
                0xa6, 0x45, 0xfe, 0x56, 0xad, 0xde, 0xfe, 0xca, 0, 0, 0, 0, 0,
                0, 0, 0,
            ],
        )
        .unwrap();
        test_encoding_roundtrip(
            &0x1adefeed65671331cafedead56fe45a6_i128,
            [
                0xa6, 0x45, 0xfe, 0x56, 0xad, 0xde, 0xfe, 0xca, 0x31, 0x13,
                0x67, 0x65, 0xed, 0xfe, 0xde, 0x1a,
            ],
        )
        .unwrap();
    }

    #[test]
    #[should_panic(expected = "ExceedMaxItems(131071)")]
    fn test_usize_encode_fail() {
        0x01FFFF_usize.strict_serialize().unwrap();
    }

    #[test]
    #[should_panic(expected = "DataNotEntirelyConsumed")]
    fn test_usize_decode_fail() {
        let _: usize = strict_deserialize([0xFF, 0xFF, 0xFF, 0x54]).unwrap();
    }

    #[test]
    fn test_bool_encoding() {
        test_encoding_roundtrip(&true, [0x01]).unwrap();
        test_encoding_roundtrip(&false, [0x00]).unwrap();

        assert_eq!(
            bool::strict_decode(&[0x20][..]),
            Err(Error::ValueOutOfRange("boolean", 0..1, 0x20))
        );
    }

    #[test]
    fn test_float_encoding() {
        test_encoding_roundtrip(&5.7692_f32, [73, 157, 184, 64]).unwrap();
        test_encoding_roundtrip(
            &54546457.76965676_f64,
            [206, 65, 40, 206, 128, 2, 138, 65],
        )
        .unwrap();
    }

    #[test]
    #[cfg(feature = "chrono")]
    fn test_chrono_encoding() {
        let utc = Utc::now();

        let ser = utc.strict_serialize().unwrap();
        assert_eq!(ser.len(), 8);

        let naive = utc.naive_utc();
        let naive = NaiveDateTime::from_timestamp(naive.timestamp(), 0);
        assert_eq!(strict_deserialize(&ser), Ok(naive));

        let ser = naive.strict_serialize().unwrap();
        assert_eq!(ser.len(), 8);
        assert_eq!(strict_deserialize(&ser), Ok(naive));

        let duration = Duration::new(naive.timestamp() as u64, 38455567);
        let ser = duration.strict_serialize().unwrap();
        assert_eq!(ser.len(), 12);
        assert_eq!(strict_deserialize(&ser), Ok(duration));
    }
}
