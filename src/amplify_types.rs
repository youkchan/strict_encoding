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

use amplify::flags::FlagVec;
use amplify::num::{u1024, u256, u512};
use std::io;

use crate::{Error, StrictDecode, StrictEncode};

impl StrictEncode for FlagVec {
    #[inline]
    fn strict_encode<E: io::Write>(&self, e: E) -> Result<usize, Error> {
        self.shrunk().as_inner().strict_encode(e)
    }
}

impl StrictDecode for FlagVec {
    #[inline]
    fn strict_decode<D: io::Read>(d: D) -> Result<Self, Error> {
        Ok(Self::from_inner(StrictDecode::strict_decode(d)?))
    }
}

impl StrictEncode for u256 {
    fn strict_encode<E: io::Write>(&self, e: E) -> Result<usize, Error> {
        self.to_le_bytes().strict_encode(e)
    }
}

impl StrictDecode for u256 {
    fn strict_decode<D: io::Read>(d: D) -> Result<Self, Error> {
        Ok(u256::from_le_bytes(<[u8; 32]>::strict_decode(d)?))
    }
}

impl StrictEncode for u512 {
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        let bytes = self.to_le_bytes();
        e.write_all(&bytes)?;
        Ok(bytes.len())
    }
}

impl StrictDecode for u512 {
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut bytes = [0u8; 64];
        d.read_exact(&mut bytes)?;
        Ok(u512::from_le_bytes(bytes))
    }
}

impl StrictEncode for u1024 {
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        let bytes = self.to_le_bytes();
        e.write_all(&bytes)?;
        Ok(bytes.len())
    }
}

impl StrictDecode for u1024 {
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut bytes = [0u8; 128];
        d.read_exact(&mut bytes)?;
        Ok(u1024::from_le_bytes(bytes))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_helpers::test_encoding_roundtrip;

    #[test]
    fn test_large_uints() {
        test_encoding_roundtrip(
            &u256::from_u64(0x_dead_cafe_4bad_beef).unwrap(),
            [
                0xef, 0xbe, 0xad, 0x4b, 0xfe, 0xca, 0xad, 0xde, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ],
        )
        .unwrap();

        test_encoding_roundtrip(
            &u512::from_u64(0x_dead_cafe_4bad_beef).unwrap(),
            [
                0xef, 0xbe, 0xad, 0x4b, 0xfe, 0xca, 0xad, 0xde, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
        )
        .unwrap();

        test_encoding_roundtrip(
            &u1024::from_u64(0x_dead_cafe_4bad_beef).unwrap(),
            [
                0xef, 0xbe, 0xad, 0x4b, 0xfe, 0xca, 0xad, 0xde, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],
        )
        .unwrap();
    }
}
