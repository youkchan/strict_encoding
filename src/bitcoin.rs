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

use std::io;

use bitcoin::bech32::u5;
use bitcoin::util::address::{self, Address};
use bitcoin::util::psbt::PartiallySignedTransaction;
use bitcoin::{
    secp256k1, util::bip32, Amount, BlockHash, OutPoint, PubkeyHash, Script,
    ScriptHash, SigHash, Transaction, TxIn, TxOut, Txid, WPubkeyHash,
    WScriptHash, Wtxid, XpubIdentifier,
};

use crate::{strategies, Error, Strategy, StrictDecode, StrictEncode};

impl Strategy for Txid {
    type Strategy = strategies::HashFixedBytes;
}
impl Strategy for Wtxid {
    type Strategy = strategies::HashFixedBytes;
}
impl Strategy for BlockHash {
    type Strategy = strategies::HashFixedBytes;
}
impl Strategy for XpubIdentifier {
    type Strategy = strategies::HashFixedBytes;
}
impl Strategy for PubkeyHash {
    type Strategy = strategies::HashFixedBytes;
}
impl Strategy for WPubkeyHash {
    type Strategy = strategies::HashFixedBytes;
}
impl Strategy for ScriptHash {
    type Strategy = strategies::HashFixedBytes;
}
impl Strategy for WScriptHash {
    type Strategy = strategies::HashFixedBytes;
}
impl Strategy for SigHash {
    type Strategy = strategies::HashFixedBytes;
}

impl StrictEncode for secp256k1::SecretKey {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        Ok(e.write(&self[..])?)
    }
}

impl StrictDecode for secp256k1::SecretKey {
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut buf = [0u8; secp256k1::constants::SECRET_KEY_SIZE];
        d.read_exact(&mut buf)?;
        Self::from_slice(&buf).map_err(|_| {
            Error::DataIntegrityError("invalid private key data".to_string())
        })
    }
}

impl StrictEncode for secp256k1::PublicKey {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        Ok(e.write(&self.serialize())?)
    }
}

impl StrictDecode for secp256k1::PublicKey {
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut buf = [0u8; secp256k1::constants::PUBLIC_KEY_SIZE];
        d.read_exact(&mut buf)?;
        if buf[0] == 0x04 {
            return Err(Error::DataIntegrityError(s!(
                "invalid public key data: uncompressed Secp256k1 public key \
                format is not allowed, use compressed form instead"
            )));
        }
        Self::from_slice(&buf).map_err(|_| {
            Error::DataIntegrityError(s!("invalid public key data"))
        })
    }
}

impl StrictEncode for secp256k1::schnorrsig::PublicKey {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        Ok(e.write(&[0x02_u8])? + e.write(&self.serialize())?)
    }
}

impl StrictDecode for secp256k1::schnorrsig::PublicKey {
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut buf =
            [0u8; secp256k1::constants::SCHNORRSIG_PUBLIC_KEY_SIZE + 1];
        d.read_exact(&mut buf)?;
        if buf[0] != 0x02 {
            return Err(Error::DataIntegrityError(s!(
                "invalid public key data: BIP340 keys must be serialized \
                with `0x02` prefix byte"
            )));
        }
        Self::from_slice(&buf[1..]).map_err(|_| {
            Error::DataIntegrityError(s!("invalid public key data"))
        })
    }
}

// TODO: #17 Implement strict encoding for `KeyPair` type once there will be a
//       way to serialize its inner data in Secpk256k1 lib (see
//       <https://github.com/rust-bitcoin/rust-secp256k1/issues/298>)

impl StrictEncode for secp256k1::Signature {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        Ok(e.write(&self.serialize_compact())?)
    }
}

impl StrictDecode for secp256k1::Signature {
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut buf = [0u8; secp256k1::constants::COMPACT_SIGNATURE_SIZE];
        d.read_exact(&mut buf)?;
        Self::from_compact(&buf).map_err(|_| {
            Error::DataIntegrityError(
                "Invalid secp256k1 ECDSA signature data".to_string(),
            )
        })
    }
}

impl StrictEncode for secp256k1::schnorrsig::Signature {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        Ok(e.write(&self[..])?)
    }
}

impl StrictDecode for secp256k1::schnorrsig::Signature {
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut buf = [0u8; secp256k1::constants::SCHNORRSIG_SIGNATURE_SIZE];
        d.read_exact(&mut buf)?;
        Self::from_slice(&buf).map_err(|_| {
            Error::DataIntegrityError(
                "Invalid secp256k1 Schnorr signature data".to_string(),
            )
        })
    }
}

#[doc(hidden)]
#[allow(useless_deprecated)]
#[deprecated(
    since = "1.4.0",
    note = "Uncompressed PublicKey serialization is not recommended, use \
            `secp256k1::PublicKey` type instead"
)]
impl StrictEncode for bitcoin::PublicKey {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        Ok(if self.compressed {
            e.write(&self.key.serialize())?
        } else {
            e.write(&self.key.serialize_uncompressed())?
        })
    }
}

#[doc(hidden)]
#[allow(useless_deprecated)]
#[deprecated(
    since = "1.4.0",
    note = "Uncompressed PublicKey serialization is not recommended, use \
            `secp256k1::PublicKey` type instead"
)]
impl StrictDecode for bitcoin::PublicKey {
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let marker = u8::strict_decode(&mut d)?;
        match marker {
            0x04 => {
                let mut buf = [0u8; secp256k1::constants::UNCOMPRESSED_PUBLIC_KEY_SIZE];
                buf[0] = marker;
                d.read_exact(&mut buf[1..])?;
                Ok(Self::from_slice(&buf).map_err(|_| {
                    Error::DataIntegrityError("Wrong public key data sequence".to_string())
                })?)
            }
            0x03 | 0x02 => {
                let mut buf = [0u8; secp256k1::constants::PUBLIC_KEY_SIZE];
                buf[0] = marker;
                d.read_exact(&mut buf[1..])?;
                Ok(Self::from_slice(&buf).map_err(|_| {
                    Error::DataIntegrityError("Wrong public key data sequence".to_string())
                })?)
            }
            invalid_flag => Err(Error::DataIntegrityError(format!(
                "Invalid public key encoding flag {:#04x}; must be either 0x02, 0x03 or 0x04",
                invalid_flag
            ))),
        }
    }
}

impl Strategy for OutPoint {
    type Strategy = strategies::BitcoinConsensus;
}
impl Strategy for TxOut {
    type Strategy = strategies::BitcoinConsensus;
}
impl Strategy for TxIn {
    type Strategy = strategies::BitcoinConsensus;
}
impl Strategy for Transaction {
    type Strategy = strategies::BitcoinConsensus;
}
impl Strategy for PartiallySignedTransaction {
    type Strategy = strategies::BitcoinConsensus;
}

impl StrictEncode for address::Payload {
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        Ok(match self {
            address::Payload::PubkeyHash(pkh) => {
                32u8.strict_encode(&mut e)? + pkh.strict_encode(&mut e)?
            }
            address::Payload::ScriptHash(sh) => {
                33u8.strict_encode(&mut e)? + sh.strict_encode(&mut e)?
            }
            address::Payload::WitnessProgram { version, program } => {
                version.to_u8().strict_encode(&mut e)?
                    + program.strict_encode(&mut e)?
            }
        })
    }
}

impl StrictDecode for address::Payload {
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        Ok(match u8::strict_decode(&mut d)? {
            32u8 => {
                address::Payload::PubkeyHash(PubkeyHash::strict_decode(&mut d)?)
            }
            33u8 => {
                address::Payload::ScriptHash(ScriptHash::strict_decode(&mut d)?)
            }
            // TODO: #18 Update to `WitnessVersion` upon bitcoin 0.26.1 release
            version if version <= 16 => address::Payload::WitnessProgram {
                version: u5::try_from_u8(version)
                    .expect("bech32::u8 decider is broken"),
                program: StrictDecode::strict_decode(&mut d)?,
            },
            wrong => {
                return Err(Error::ValueOutOfRange(
                    "witness program version",
                    0..17,
                    wrong as u128,
                ))
            }
        })
    }
}

impl StrictEncode for Address {
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        Ok(strict_encode_list!(e; self.network, self.payload))
    }
}

impl StrictDecode for Address {
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        Ok(strict_decode_self!(d; network, payload; crate))
    }
}

impl StrictEncode for Amount {
    fn strict_encode<E: io::Write>(&self, e: E) -> Result<usize, Error> {
        self.as_sat().strict_encode(e)
    }
}

impl StrictDecode for Amount {
    fn strict_decode<D: io::Read>(d: D) -> Result<Self, Error> {
        Ok(Amount::from_sat(u64::strict_decode(d)?))
    }
}

impl StrictEncode for Script {
    #[inline]
    fn strict_encode<E: io::Write>(&self, e: E) -> Result<usize, Error> {
        self.to_bytes().strict_encode(e)
    }
}

impl StrictDecode for Script {
    #[inline]
    fn strict_decode<D: io::Read>(d: D) -> Result<Self, Error> {
        Ok(Self::from(Vec::<u8>::strict_decode(d)?))
    }
}

impl StrictEncode for bitcoin::Network {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        self.magic().strict_encode(&mut e)
    }
}

impl StrictDecode for bitcoin::Network {
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let magic = u32::strict_decode(&mut d)?;
        Self::from_magic(magic).ok_or(Error::ValueOutOfRange(
            "bitcoin::Network",
            0..0,
            magic as u128,
        ))
    }
}

impl StrictEncode for bip32::ChildNumber {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        let (t, index) = match self {
            bip32::ChildNumber::Normal { index } => (0u8, index),
            bip32::ChildNumber::Hardened { index } => (1u8, index),
        };
        Ok(strict_encode_list!(e; t, index))
    }
}

impl StrictDecode for bip32::ChildNumber {
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let t = u8::strict_decode(&mut d)?;
        let index = u32::strict_decode(&mut d)?;
        match t {
            0 => Ok(bip32::ChildNumber::Normal { index }),
            1 => Ok(bip32::ChildNumber::Hardened { index }),
            x => {
                Err(Error::EnumValueNotKnown("bip32::ChildNumber", x as usize))
            }
        }
    }
}

impl StrictEncode for bip32::DerivationPath {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        let buf: Vec<bip32::ChildNumber> =
            self.into_iter().map(bip32::ChildNumber::clone).collect();
        buf.strict_encode(&mut e)
    }
}

impl StrictDecode for bip32::DerivationPath {
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        Ok(Self::from(Vec::<bip32::ChildNumber>::strict_decode(
            &mut d,
        )?))
    }
}

impl StrictEncode for bip32::ChainCode {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        Ok(e.write(self.as_bytes())?)
    }
}

impl StrictDecode for bip32::ChainCode {
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut buf = [0u8; 32];
        d.read_exact(&mut buf)?;
        Ok(Self::from(&buf[..]))
    }
}

impl StrictEncode for bip32::Fingerprint {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        Ok(e.write(self.as_bytes())?)
    }
}

impl StrictDecode for bip32::Fingerprint {
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut buf = [0u8; 4];
        d.read_exact(&mut buf)?;
        Ok(Self::from(&buf[..]))
    }
}

impl StrictEncode for bip32::ExtendedPubKey {
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        Ok(e.write(&self.encode())?)
    }
}

impl StrictDecode for bip32::ExtendedPubKey {
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut buf = [0u8; 78];
        d.read_exact(&mut buf)?;
        bip32::ExtendedPubKey::decode(&buf).map_err(|_| {
            Error::DataIntegrityError(
                "Extended pubkey integrity is broken".to_string(),
            )
        })
    }
}

impl StrictEncode for bip32::ExtendedPrivKey {
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        Ok(e.write(&self.encode())?)
    }
}

impl StrictDecode for bip32::ExtendedPrivKey {
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut buf = [0u8; 78];
        d.read_exact(&mut buf)?;
        bip32::ExtendedPrivKey::decode(&buf).map_err(|_| {
            Error::DataIntegrityError(
                "Extended privkey integrity is broken".to_string(),
            )
        })
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::str::FromStr;

    use bitcoin::{
        consensus, hashes::hex::FromHex, hashes::Hash, secp256k1::Message,
    };
    use bitcoin_hashes::{hash160, hmac, ripemd160, sha256, sha256d, sha256t};

    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn test_encoding_hashes() {
        static HASH256_BYTES: [u8; 32] = [
            0x15, 0x2d, 0x1c, 0x97, 0x61, 0xd4, 0x64, 0x66, 0x68, 0xdf, 0xcd,
            0xeb, 0x11, 0x98, 0x70, 0x84, 0x4e, 0xdb, 0x25, 0xa0, 0xea, 0x1e,
            0x35, 0x20, 0x7f, 0xaa, 0x44, 0xa9, 0x67, 0xa6, 0xa6, 0x61,
        ];
        static HASH160_BYTES: [u8; 20] = [
            0x15, 0x2d, 0x1c, 0x97, 0x61, 0xd4, 0x64, 0x66, 0x68, 0xdf, 0xcd,
            0xeb, 0x11, 0x98, 0x4e, 0xdb, 0x25, 0xa0, 0xea, 0x1e,
        ];

        const TEST_MIDSTATE: [u8; 32] = [
            156, 224, 228, 230, 124, 17, 108, 57, 56, 179, 202, 242, 195, 15,
            80, 137, 211, 243, 147, 108, 71, 99, 110, 96, 125, 179, 62, 234,
            221, 198, 240, 201,
        ];

        #[derive(
            Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash,
        )]
        pub struct TestHashTag;

        impl sha256t::Tag for TestHashTag {
            fn engine() -> sha256::HashEngine {
                // The TapRoot TapLeaf midstate.
                let midstate = sha256::Midstate::from_inner(TEST_MIDSTATE);
                sha256::HashEngine::from_midstate(midstate, 64)
            }
        }

        test_encoding_roundtrip(
            &ripemd160::Hash::from_inner(HASH160_BYTES),
            HASH160_BYTES,
        )
        .unwrap();
        test_encoding_roundtrip(
            &hash160::Hash::from_inner(HASH160_BYTES),
            HASH160_BYTES,
        )
        .unwrap();
        test_encoding_roundtrip(
            &hmac::Hmac::<sha256::Hash>::from_inner(HASH256_BYTES),
            HASH256_BYTES,
        )
        .unwrap();
        test_encoding_roundtrip(
            &sha256::Hash::from_inner(HASH256_BYTES),
            HASH256_BYTES,
        )
        .unwrap();
        test_encoding_roundtrip(
            &sha256d::Hash::from_inner(HASH256_BYTES),
            HASH256_BYTES,
        )
        .unwrap();
        test_encoding_roundtrip(
            &sha256t::Hash::<TestHashTag>::from_inner(HASH256_BYTES),
            HASH256_BYTES,
        )
        .unwrap();
        test_encoding_roundtrip(
            &Txid::from_slice(&HASH256_BYTES).unwrap(),
            HASH256_BYTES,
        )
        .unwrap();
        test_encoding_roundtrip(
            &Wtxid::from_slice(&HASH256_BYTES).unwrap(),
            HASH256_BYTES,
        )
        .unwrap();
        test_encoding_roundtrip(
            &BlockHash::from_slice(&HASH256_BYTES).unwrap(),
            HASH256_BYTES,
        )
        .unwrap();
        test_encoding_roundtrip(
            &XpubIdentifier::from_slice(&HASH160_BYTES).unwrap(),
            HASH160_BYTES,
        )
        .unwrap();
        test_encoding_roundtrip(
            &SigHash::from_slice(&HASH256_BYTES).unwrap(),
            HASH256_BYTES,
        )
        .unwrap();
        test_encoding_roundtrip(
            &PubkeyHash::from_slice(&HASH160_BYTES).unwrap(),
            HASH160_BYTES,
        )
        .unwrap();
        test_encoding_roundtrip(
            &WPubkeyHash::from_slice(&HASH160_BYTES).unwrap(),
            HASH160_BYTES,
        )
        .unwrap();
        test_encoding_roundtrip(
            &ScriptHash::from_slice(&HASH160_BYTES).unwrap(),
            HASH160_BYTES,
        )
        .unwrap();
        test_encoding_roundtrip(
            &WScriptHash::from_slice(&HASH256_BYTES).unwrap(),
            HASH256_BYTES,
        )
        .unwrap();
    }

    #[test]
    fn test_encoding_seckey(
    ) -> Result<(), DataEncodingTestFailure<secp256k1::SecretKey>> {
        let secp = secp256k1::Secp256k1::new();
        static SK_BYTES: [u8; 32] = [
            0x15, 0x2d, 0x1c, 0x97, 0x61, 0xd4, 0x64, 0x66, 0x68, 0xdf, 0xcd,
            0xeb, 0x11, 0x98, 0x70, 0x84, 0x4e, 0xdb, 0x25, 0xa0, 0xea, 0x1e,
            0x35, 0x20, 0x7f, 0xaa, 0x44, 0xa9, 0x67, 0xa6, 0xa6, 0x61,
        ];
        let sk = secp256k1::SecretKey::from_slice(&SK_BYTES).unwrap();
        let _sk_bip340 =
            secp256k1::schnorrsig::KeyPair::from_seckey_slice(&secp, &SK_BYTES)
                .unwrap();
        // TODO: #17 implement KeyPair serialization testing
        test_encoding_roundtrip(&sk, &SK_BYTES[..])
    }

    #[test]
    fn test_encoding_pubkey() {
        static PK_BYTES_02: [u8; 33] = [
            0x02, 0x9b, 0x63, 0x47, 0x39, 0x85, 0x05, 0xf5, 0xec, 0x93, 0x82,
            0x6d, 0xc6, 0x1c, 0x19, 0xf4, 0x7c, 0x66, 0xc0, 0x28, 0x3e, 0xe9,
            0xbe, 0x98, 0x0e, 0x29, 0xce, 0x32, 0x5a, 0x0f, 0x46, 0x79, 0xef,
        ];
        static PK_BYTES_03: [u8; 33] = [
            0x03, 0x9b, 0x63, 0x47, 0x39, 0x85, 0x05, 0xf5, 0xec, 0x93, 0x82,
            0x6d, 0xc6, 0x1c, 0x19, 0xf4, 0x7c, 0x66, 0xc0, 0x28, 0x3e, 0xe9,
            0xbe, 0x98, 0x0e, 0x29, 0xce, 0x32, 0x5a, 0x0f, 0x46, 0x79, 0xef,
        ];
        static PK_BYTES_04: [u8; 65] = [
            0x04, 0x9b, 0x63, 0x47, 0x39, 0x85, 0x05, 0xf5, 0xec, 0x93, 0x82,
            0x6d, 0xc6, 0x1c, 0x19, 0xf4, 0x7c, 0x66, 0xc0, 0x28, 0x3e, 0xe9,
            0xbe, 0x98, 0x0e, 0x29, 0xce, 0x32, 0x5a, 0x0f, 0x46, 0x79, 0xef,
            0x87, 0x28, 0x8e, 0xd7, 0x3c, 0xe4, 0x7f, 0xc4, 0xf5, 0xc7, 0x9d,
            0x19, 0xeb, 0xfa, 0x57, 0xda, 0x7c, 0xff, 0x3a, 0xff, 0x6e, 0x81,
            0x9e, 0x4e, 0xe9, 0x71, 0xd8, 0x6b, 0x5e, 0x61, 0x87, 0x5d,
        ];
        static PK_BYTES_ONEKEY: [u8; 33] = [
            0x02, 0x79, 0xbe, 0x66, 0x7e, 0xf9, 0xdc, 0xbb, 0xac, 0x55, 0xa0,
            0x62, 0x95, 0xce, 0x87, 0x0b, 0x07, 0x02, 0x9b, 0xfc, 0xdb, 0x2d,
            0xce, 0x28, 0xd9, 0x59, 0xf2, 0x81, 0x5b, 0x16, 0xf8, 0x17, 0x98,
        ];

        let secp_pk_02 =
            secp256k1::PublicKey::from_slice(&PK_BYTES_02).unwrap();
        let secp_pk_03 =
            secp256k1::PublicKey::from_slice(&PK_BYTES_03).unwrap();
        let secp_pk_one =
            secp256k1::PublicKey::from_slice(&PK_BYTES_ONEKEY).unwrap();
        test_encoding_roundtrip(&secp_pk_02, PK_BYTES_02).unwrap();
        test_encoding_roundtrip(&secp_pk_03, PK_BYTES_03).unwrap();
        test_encoding_roundtrip(&secp_pk_one, PK_BYTES_ONEKEY).unwrap();
        assert_eq!(
            secp256k1::PublicKey::strict_deserialize(&PK_BYTES_04),
            Err(Error::DataIntegrityError(s!(
                "invalid public key data: uncompressed Secp256k1 public key \
                format is not allowed, use compressed form instead"
            )))
        );

        let sk_one = secp256k1::PublicKey::from_secret_key(
            &secp256k1::Secp256k1::new(),
            &secp256k1::key::ONE_KEY,
        );

        let pubkey_02 = bitcoin::PublicKey::from_slice(&PK_BYTES_02).unwrap();
        let pubkey_03 = bitcoin::PublicKey::from_slice(&PK_BYTES_03).unwrap();
        let pubkey_04 = bitcoin::PublicKey::from_slice(&PK_BYTES_04).unwrap();
        let one_key = bitcoin::PublicKey {
            compressed: true,
            key: sk_one,
        };
        test_encoding_roundtrip(&pubkey_02, PK_BYTES_02).unwrap();
        test_encoding_roundtrip(&pubkey_03, PK_BYTES_03).unwrap();
        test_encoding_roundtrip(&pubkey_04, PK_BYTES_04).unwrap();
        test_encoding_roundtrip(&one_key, PK_BYTES_ONEKEY).unwrap();
        assert_eq!(secp_pk_02, pubkey_02.key);
        assert_eq!(secp_pk_02, pubkey_02.key);
        assert_eq!(secp_pk_02, pubkey_02.key);
        assert_eq!(secp_pk_03, pubkey_03.key);
        assert_eq!(secp_pk_one, one_key.key);
        assert_eq!(pubkey_03.key, pubkey_04.key);

        let xcoordonly_02 =
            secp256k1::schnorrsig::PublicKey::from_slice(&PK_BYTES_02[1..])
                .unwrap();
        let xcoordonly_one =
            secp256k1::schnorrsig::PublicKey::from_slice(&PK_BYTES_ONEKEY[1..])
                .unwrap();
        test_encoding_roundtrip(&xcoordonly_02, PK_BYTES_02).unwrap();
        test_encoding_roundtrip(&xcoordonly_one, PK_BYTES_ONEKEY).unwrap();
        assert_eq!(
            secp256k1::schnorrsig::PublicKey::strict_decode(&PK_BYTES_03[..]),
            Err(Error::DataIntegrityError(s!(
                "invalid public key data: BIP340 keys must be serialized \
                with `0x02` prefix byte"
            )))
        );
        assert_eq!(
            secp256k1::schnorrsig::PublicKey::strict_decode(&PK_BYTES_04[..]),
            Err(Error::DataIntegrityError(s!(
                "invalid public key data: BIP340 keys must be serialized \
                with `0x02` prefix byte"
            )))
        );
        assert_eq!(xcoordonly_02.serialize(), secp_pk_02.serialize()[1..]);
        assert_eq!(xcoordonly_02.serialize(), secp_pk_03.serialize()[1..]);
        assert_eq!(xcoordonly_one.serialize(), secp_pk_one.serialize()[1..]);
    }

    #[test]
    #[should_panic(expected = "UnexpectedEof")]
    fn test_garbagedata_pubkey() {
        static PK_BYTES_04: [u8; 60] = [
            0x04, 0x9b, 0x63, 0x47, 0x39, 0x85, 0x05, 0xf5, 0xec, 0x93, 0x82,
            0x6d, 0xc6, 0x1c, 0x19, 0xf4, 0x7c, 0x66, 0xc0, 0x28, 0x3e, 0xe9,
            0xbe, 0x98, 0x0e, 0x29, 0xce, 0x32, 0x5a, 0x0f, 0x46, 0x79, 0xef,
            0x87, 0x28, 0x8e, 0xd7, 0x3c, 0xe4, 0x7f, 0xc4, 0xf5, 0xc7, 0x9d,
            0x19, 0xeb, 0xfa, 0x57, 0xda, 0x7c, 0xff, 0x3a, 0xff, 0x6e, 0x81,
            0x9e, 0x4e, 0xe9, 0x71, 0xd8,
        ];
        bitcoin::PublicKey::strict_decode(&PK_BYTES_04[..]).unwrap();
    }

    #[test]
    #[should_panic(expected = "DataIntegrityError")]
    fn test_grabagedata_pubkey2() {
        static PK_BYTES_02: [u8; 33] = [
            0xa5, 0x9b, 0x63, 0x47, 0x39, 0x85, 0x05, 0xf5, 0xec, 0x93, 0x82,
            0x6d, 0xc6, 0x1c, 0x19, 0xf4, 0x7c, 0x66, 0xc0, 0x28, 0x3e, 0xe9,
            0xbe, 0x98, 0x0e, 0x29, 0xce, 0x32, 0x5a, 0x0f, 0x46, 0x79, 0xef,
        ];
        bitcoin::PublicKey::strict_decode(&PK_BYTES_02[..]).unwrap();
    }

    static ECDSA_BYTES: [u8; 64] = [
        0xdf, 0x2b, 0x07, 0x01, 0x5f, 0x2e, 0x01, 0x67, 0x74, 0x18, 0x7e, 0xad,
        0x4a, 0x4f, 0x71, 0x9a, 0x14, 0xe3, 0xe1, 0xad, 0xa1, 0x78, 0xd6, 0x6c,
        0xce, 0xcf, 0xa4, 0x5b, 0x63, 0x30, 0x70, 0xc2, 0x43, 0xa2, 0xd7, 0x6e,
        0xe0, 0x5d, 0x63, 0x49, 0xfe, 0x98, 0x69, 0x6c, 0x1c, 0x4d, 0x9a, 0x67,
        0x11, 0x24, 0xde, 0x40, 0xc5, 0x31, 0x71, 0xa4, 0xb2, 0x82, 0xb7, 0x69,
        0xb7, 0xc6, 0x96, 0xcd,
    ];

    static SCHNORR_BYTES: [u8; 64] = [
        0x16, 0x2A, 0xDD, 0x99, 0x90, 0x6A, 0x93, 0x04, 0xD4, 0xD0, 0x79, 0x02,
        0x73, 0xAA, 0xD1, 0xB3, 0x54, 0x13, 0x6C, 0x32, 0xBE, 0x53, 0xF0, 0x7D,
        0xF8, 0x0F, 0x74, 0xB3, 0xF7, 0xBF, 0x99, 0xAE, 0x91, 0x13, 0x45, 0x76,
        0xD0, 0x4B, 0x8A, 0x2D, 0x82, 0xDA, 0x7F, 0xE9, 0xDA, 0x43, 0x99, 0xF3,
        0xBD, 0x99, 0xDF, 0x6B, 0xA3, 0xC6, 0xF9, 0xB4, 0xFB, 0x5E, 0x62, 0xDB,
        0x1B, 0x15, 0xCF, 0xF3,
    ];

    #[test]
    fn test_encode_signatures() {
        let secp = secp256k1::Secp256k1::new();

        static KEY: [u8; 32] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48,
            0xA0, 0x3B, 0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36, 0x41, 0x40,
        ];

        let sk_ecdsa = secp256k1::SecretKey::from_slice(&KEY).unwrap();
        let sk_schnorr =
            secp256k1::schnorrsig::KeyPair::from_seckey_slice(&secp, &KEY)
                .unwrap();

        let pk_ecdsa = secp256k1::PublicKey::from_secret_key(&secp, &sk_ecdsa);
        let pk_schnorr =
            secp256k1::schnorrsig::PublicKey::from_keypair(&secp, &sk_schnorr);
        let msg = Message::from_slice(&[1u8; 32]).unwrap();

        let ecdsa = secp.sign(&msg, &sk_ecdsa);
        test_encoding_roundtrip(&ecdsa, &ECDSA_BYTES).unwrap();
        assert!(secp.verify(&msg, &ecdsa, &pk_ecdsa).is_ok());

        let schnorr = secp.schnorrsig_sign_no_aux_rand(&msg, &sk_schnorr);
        test_encoding_roundtrip(&schnorr, &SCHNORR_BYTES).unwrap();
        assert!(secp.schnorrsig_verify(&schnorr, &msg, &pk_schnorr).is_ok());

        // Schnorr signature can be deserialized as ECDSA and vice verse,
        // (since there is no encoding-level way of verifying its type)
        // but MUST be invalid upon signature validation
        let schnorr_as_ecdsa: secp256k1::Signature =
            test_vec_decoding_roundtrip(&SCHNORR_BYTES).unwrap();
        let ecdsa_as_schnorr: secp256k1::schnorrsig::Signature =
            test_vec_decoding_roundtrip(&ECDSA_BYTES).unwrap();
        assert_eq!(
            secp.verify(&msg, &schnorr_as_ecdsa, &pk_ecdsa),
            Err(secp256k1::Error::IncorrectSignature)
        );
        assert_eq!(
            secp.schnorrsig_verify(&ecdsa_as_schnorr, &msg, &pk_schnorr),
            Err(secp256k1::Error::InvalidSignature)
        );
    }

    #[test]
    #[should_panic(expected = "UnexpectedEof")]
    fn test_garbagedata_ecdsa() {
        secp256k1::Signature::strict_decode(&ECDSA_BYTES[5..]).unwrap();
    }

    #[test]
    #[should_panic(expected = "UnexpectedEof")]
    fn test_garbagedata_schnorrsig() {
        secp256k1::schnorrsig::Signature::strict_decode(&SCHNORR_BYTES[5..])
            .unwrap();
    }

    #[test]
    fn test_encoding_network(
    ) -> Result<(), DataEncodingTestFailure<bitcoin::Network>> {
        test_encoding_roundtrip(
            &bitcoin::Network::Bitcoin,
            &[0xF9, 0xBE, 0xB4, 0xD9],
        )?;
        test_encoding_roundtrip(
            &bitcoin::Network::Testnet,
            &[0x0B, 0x11, 0x09, 0x07],
        )?;
        test_encoding_roundtrip(
            &bitcoin::Network::Signet,
            &[0x0A, 0x03, 0xCF, 0x40],
        )?;
        test_encoding_roundtrip(
            &bitcoin::Network::Regtest,
            &[0xFA, 0xBF, 0xB5, 0xDA],
        )
    }

    #[test]
    #[should_panic(
        expected = r#"ValueOutOfRange("bitcoin::Network", 0..0, 2762187425)"#
    )]
    fn test_encoding_network_failure() {
        // Bitcoin Network structure do not support "Other" networks
        bitcoin::Network::strict_decode(&[0xA1u8, 0xA2u8, 0xA3u8, 0xA4u8][..])
            .unwrap();
    }

    #[test]
    fn test_encoding_address() {
        test_encoding_roundtrip(
            &Address::from_str("12CL4K2eVqj7hQTix7dM7CVHCkpP17Pry3").unwrap(),
            [
                0xF9, 0xBE, 0xB4, 0xD9, 0x20, 0x0D, 0x1C, 0x9C, 0x02, 0xA7,
                0xBE, 0x9B, 0xA8, 0xB8, 0x84, 0x28, 0x04, 0xFE, 0xB9, 0x61,
                0x48, 0x1C, 0xE6, 0x56, 0x1B,
            ],
        )
        .unwrap();
        test_encoding_roundtrip(
            &Address::from_str("3AfyxhpBVVLmBR4ZYX2onGzRqjv5QZ7FqD").unwrap(),
            [
                0xF9, 0xBE, 0xB4, 0xD9, 0x21, 0x62, 0x87, 0x13, 0xE2, 0x7A,
                0x36, 0xDA, 0x16, 0x17, 0x4E, 0x9D, 0x02, 0xC1, 0x77, 0x2C,
                0xD9, 0xE4, 0x06, 0x03, 0x9B,
            ],
        )
        .unwrap();
        test_encoding_roundtrip(
            &Address::from_str("bc1qp5wfcq48h6d63wyy9qz0awtpfqwwv4sma86mhz")
                .unwrap(),
            [
                0xF9, 0xBE, 0xB4, 0xD9, 0x00, 0x14, 0x00, 0x0D, 0x1C, 0x9C,
                0x02, 0xA7, 0xBE, 0x9B, 0xA8, 0xB8, 0x84, 0x28, 0x04, 0xFE,
                0xB9, 0x61, 0x48, 0x1C, 0xE6, 0x56, 0x1B,
            ],
        )
        .unwrap();

        static P2WSH_BC: [u8; 39] = [
            0xF9, 0xBE, 0xB4, 0xD9, 0x00, 0x20, 0x00, 0x18, 0x63, 0x14, 0x3C,
            0x14, 0xC5, 0x16, 0x68, 0x04, 0xBD, 0x19, 0x20, 0x33, 0x56, 0xDA,
            0x13, 0x6C, 0x98, 0x56, 0x78, 0xCD, 0x4D, 0x27, 0xA1, 0xB8, 0xC6,
            0x32, 0x96, 0x04, 0x90, 0x32, 0x62,
        ];
        test_encoding_roundtrip(
            &Address::from_str("bc1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3qccfmv3")
                .unwrap(),
            P2WSH_BC,
        ).unwrap();
        // TODO: #18 test_encoding_roundtrip(&Address::from_str("bc1pw508d6qejxtdg4y5r3zarvary0c5xw7kw508d6qejxtdg4y5r3zarvary0c5xw7kt5nd6y").unwrap(), []).unwrap();
        test_encoding_roundtrip(
            &Address::from_str("mgiHMN7dJsANUWwLfgbiw7hc4kR5xMjPhw").unwrap(),
            [
                0x0B, 0x11, 0x09, 0x07, 0x20, 0x0D, 0x1C, 0x9C, 0x02, 0xA7,
                0xBE, 0x9B, 0xA8, 0xB8, 0x84, 0x28, 0x04, 0xFE, 0xB9, 0x61,
                0x48, 0x1C, 0xE6, 0x56, 0x1B,
            ],
        )
        .unwrap();
        test_encoding_roundtrip(
            &Address::from_str("2N2EC2SkD6wr7PCh7DeegQDyh468FA7TK3a").unwrap(),
            [
                0x0B, 0x11, 0x09, 0x07, 0x21, 0x62, 0x87, 0x13, 0xE2, 0x7A,
                0x36, 0xDA, 0x16, 0x17, 0x4E, 0x9D, 0x02, 0xC1, 0x77, 0x2C,
                0xD9, 0xE4, 0x06, 0x03, 0x9B,
            ],
        )
        .unwrap();
        test_encoding_roundtrip(
            &Address::from_str("tb1qp5wfcq48h6d63wyy9qz0awtpfqwwv4smhppgv3")
                .unwrap(),
            [
                0x0B, 0x11, 0x09, 0x07, 0x00, 0x14, 0x00, 0x0D, 0x1C, 0x9C,
                0x02, 0xA7, 0xBE, 0x9B, 0xA8, 0xB8, 0x84, 0x28, 0x04, 0xFE,
                0xB9, 0x61, 0x48, 0x1C, 0xE6, 0x56, 0x1B,
            ],
        )
        .unwrap();
        static P2WSH_TB: [u8; 39] = [
            0x0B, 0x11, 0x09, 0x07, 0x00, 0x20, 0x00, 0x18, 0x63, 0x14, 0x3C,
            0x14, 0xC5, 0x16, 0x68, 0x04, 0xBD, 0x19, 0x20, 0x33, 0x56, 0xDA,
            0x13, 0x6C, 0x98, 0x56, 0x78, 0xCD, 0x4D, 0x27, 0xA1, 0xB8, 0xC6,
            0x32, 0x96, 0x04, 0x90, 0x32, 0x62,
        ];
        test_encoding_roundtrip(
            &Address::from_str("tb1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3q0sl5k7")
                .unwrap(),
            P2WSH_TB,
        ).unwrap();
        // TODO: #18 test_encoding_roundtrip(&Address::from_str("
        // tb1pqqqqp399et2xygdj5xreqhjjvcmzhxw4aywxecjdzew6hylgvsesf3hn0c").
        // unwrap(), []).unwrap();
        test_encoding_roundtrip(
            &Address::from_str("bcrt1qs758ursh4q9z627kt3pp5yysm78ddny6txaqgw")
                .unwrap(),
            [
                0xFA, 0xBF, 0xB5, 0xDA, 0x00, 0x14, 0x00, 0x87, 0xA8, 0x7E,
                0x0E, 0x17, 0xA8, 0x0A, 0x2D, 0x2B, 0xD6, 0x5C, 0x42, 0x1A,
                0x10, 0x90, 0xDF, 0x8E, 0xD6, 0xCC, 0x9A,
            ],
        )
        .unwrap();
    }

    #[test]
    #[should_panic(
        expected = r#"ValueOutOfRange("witness program version", 0..17, 35)"#
    )]
    fn test_encoding_address_failure() {
        // Address string with witness version byte (fifth) > 17 and not
        // matching non-witness address type
        Address::strict_deserialize([
            0x0B, 0x11, 0x09, 0x07, 0x23, 0x14, 0x00, 0x0D, 0x1C, 0x9C, 0x02,
            0xA7, 0xBE, 0x9B, 0xA8, 0xB8, 0x84, 0x28, 0x04, 0xFE, 0xB9, 0x61,
            0x48, 0x1C, 0xE6, 0x56, 0x1B,
        ])
        .unwrap();
    }

    #[test]
    fn test_encoding_outpoint() {
        static OUTPOINT: [u8; 36] = [
            0x53, 0xc6, 0x31, 0x13, 0xed, 0x18, 0x68, 0xfc, 0xa, 0xdf, 0x8e,
            0xcd, 0xfd, 0x1f, 0x4d, 0xd6, 0xe5, 0xe3, 0x85, 0x83, 0xa4, 0x9d,
            0xb, 0x14, 0xe7, 0xf8, 0x87, 0xa4, 0xd1, 0x61, 0x78, 0x21, 0x4,
            0x0, 0x0, 0x0,
        ];
        static OUTPOINT_NULL: [u8; 36] = [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xff, 0xff, 0xff, 0xff,
        ];

        let txid = Txid::from_hex(
            "217861d1a487f8e7140b9da48385e3e5d64d1ffdcd8edf0afc6818ed1331c653",
        )
        .unwrap();
        let vout = 4u32;

        // test random and null outpoints
        let outpoint = OutPoint::new(txid, vout);
        let _ = test_encoding_roundtrip(&outpoint, &OUTPOINT).unwrap();
        let null = OutPoint::null();
        let _ = test_encoding_roundtrip(&null, &OUTPOINT_NULL).unwrap();
    }

    #[test]
    #[should_panic(expected = "UnexpectedEof")]
    fn test_garbagedata_outpoint() {
        static OUTPOINT: [u8; 32] = [
            0x53, 0xc6, 0x31, 0x13, 0xed, 0x18, 0x68, 0xfc, 0xa, 0xdf, 0x8e,
            0xcd, 0xfd, 0x1f, 0x4d, 0xd6, 0xe5, 0xe3, 0x85, 0x83, 0xa4, 0x9d,
            0xb, 0x14, 0xe7, 0xf8, 0x87, 0xa4, 0xd1, 0x61, 0x78, 0x21,
        ];
        OutPoint::strict_decode(&OUTPOINT[..]).unwrap();
    }

    #[test]
    fn test_amount() {
        let value = 19_356_465_2435_5767__u64;
        let amount = Amount::from_sat(value);
        let data = value.to_le_bytes();
        test_encoding_roundtrip(&value, data).unwrap();
        test_encoding_roundtrip(&amount, data).unwrap();
    }

    #[test]
    fn test_tx() {
        let tx_segwit_bytes = Vec::from_hex(
            "02000000000101595895ea20179de87052b4046dfe6fd515860505d6511a9004cf\
            12a1f93cac7c0100000000ffffffff01deb807000000000017a9140f3444e271620\
            c736808aa7b33e370bd87cb5a078702483045022100fb60dad8df4af2841adc0346\
            638c16d0b8035f5e3f3753b88db122e70c79f9370220756e6633b17fd2710e62634\
            7d28d60b0a2d6cbb41de51740644b9fb3ba7751040121028fa937ca8cba2197a37c\
            007176ed8941055d3bcb8627d085e94553e62f057dcc00000000"
        ).unwrap();
        let tx_legacy1_bytes = Vec::from_hex(
            "ffffff7f0100000000000000000000000000000000000000000000000000000000\
            000000000000000000ffffffff0100f2052a01000000434104678afdb0fe5548271\
            967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f355\
            04e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000"
        ).unwrap();
        let tx_legacy2_bytes = Vec::from_hex(
            "000000800100000000000000000000000000000000000000000000000000000000\
            000000000000000000ffffffff0100f2052a01000000434104678afdb0fe5548271\
            967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f355\
            04e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000"
        ).unwrap();

        let tx_segwit: Transaction =
            consensus::deserialize(&tx_segwit_bytes).unwrap();
        let tx_legacy1: Transaction =
            consensus::deserialize(&tx_legacy1_bytes).unwrap();
        let tx_legacy2: Transaction =
            consensus::deserialize(&tx_legacy2_bytes).unwrap();

        test_encoding_roundtrip(&tx_segwit, &tx_segwit_bytes).unwrap();
        test_encoding_roundtrip(&tx_legacy1, &tx_legacy1_bytes).unwrap();
        test_encoding_roundtrip(&tx_legacy2, &tx_legacy2_bytes).unwrap();
    }

    #[test]
    fn test_txin() {
        let txin_bytes = Vec::from_hex(
            "a15d57094aa7a21a28cb20b59aab8fc7d1149a3bdbcddba9c622e4f5f6a99ece01\
            0000006c493046022100f93bb0e7d8db7bd46e40132d1f8242026e045f03a0efe71\
            bbb8e3f475e970d790221009337cd7f1f929f00cc6ff01f03729b069a7c21b59b17\
            36ddfee5db5946c5da8c0121033b9b137ee87d5a812d6f506efdd37f0affa7ffc31\
            0711c06c7f3e097c9447c52ffffffff"
        ).unwrap();
        let txin: TxIn = consensus::deserialize(&txin_bytes).unwrap();
        test_encoding_roundtrip(&txin, &txin_bytes).unwrap();
    }

    #[test]
    fn test_txout() {
        let txout_segwit_bytes = Vec::from_hex(
            "0000000000000000160014d9a1665bea770cb6ec4809943f1e8ad67a31191f",
        )
        .unwrap();
        let txout_legacy_bytes = Vec::from_hex(
            "000000000000000017a91413f5fb72e7a31fcac98df27c77217b02abdb47fd87",
        )
        .unwrap();

        let txout_segwit: TxOut =
            consensus::deserialize(&txout_segwit_bytes).unwrap();
        let txout_legacy: TxOut =
            consensus::deserialize(&txout_legacy_bytes).unwrap();

        test_encoding_roundtrip(&txout_segwit, &txout_segwit_bytes).unwrap();
        test_encoding_roundtrip(&txout_legacy, &txout_legacy_bytes).unwrap();
    }

    #[test]
    fn test_psbt() {
        let psbt_bytes = Vec::from_hex(
            "70736274ff0100750200000001268171371edff285e937adeea4b37b78000c0566\
            cbb3ad64641713ca42171bf60000000000feffffff02d3dff505000000001976a91\
            4d0c59903c5bac2868760e90fd521a4665aa7652088ac00e1f5050000000017a914\
            3545e6e33b832c47050f24d3eeb93c9c03948bc787b32e1300000100fda50101000\
            00000010289a3c71eab4d20e0371bbba4cc698fa295c9463afa2e397f8533ccb62f\
            9567e50100000017160014be18d152a9b012039daf3da7de4f53349eecb985fffff\
            fff86f8aa43a71dff1448893a530a7237ef6b4608bbb2dd2d0171e63aec6a4890b4\
            0100000017160014fe3e9ef1a745e974d902c4355943abcb34bd5353ffffffff020\
            0c2eb0b000000001976a91485cff1097fd9e008bb34af709c62197b38978a4888ac\
            72fef84e2c00000017a914339725ba21efd62ac753a9bcd067d6c7a6a39d0587024\
            7304402202712be22e0270f394f568311dc7ca9a68970b8025fdd3b240229f07f8a\
            5f3a240220018b38d7dcd314e734c9276bd6fb40f673325bc4baa144c800d2f2f02\
            db2765c012103d2e15674941bad4a996372cb87e1856d3652606d98562fe39c5e9e\
            7e413f210502483045022100d12b852d85dcd961d2f5f4ab660654df6eedcc794c0\
            c33ce5cc309ffb5fce58d022067338a8e0e1725c197fb1a88af59f51e44e4255b20\
            167c8684031c05d1f2592a01210223b72beef0965d10be0778efecd61fcac6f79a4\
            ea169393380734464f84f2ab300000000000000"
        ).unwrap();

        let psbt: PartiallySignedTransaction =
            consensus::deserialize(&psbt_bytes).unwrap();

        test_encoding_roundtrip(&psbt, &psbt_bytes).unwrap();
    }

    #[test]
    fn test_encoding_extendedpubkey() {
        static EXT_PUBKEY1: [u8; 78] = [
            4, 136, 178, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 135, 61, 255, 129, 192,
            47, 82, 86, 35, 253, 31, 229, 22, 126, 172, 58, 85, 160, 73, 222,
            61, 49, 75, 180, 46, 226, 39, 255, 237, 55, 213, 8, 3, 57, 163, 96,
            19, 48, 21, 151, 218, 239, 65, 251, 229, 147, 160, 44, 197, 19,
            208, 181, 85, 39, 236, 45, 241, 5, 14, 46, 143, 244, 156, 133, 194,
        ];

        static EXT_PUBKEY2: [u8; 78] = [
            4, 136, 178, 30, 3, 190, 245, 162, 249, 128, 0, 0, 2, 4, 70, 107,
            156, 200, 225, 97, 233, 102, 64, 156, 165, 41, 134, 197, 132, 240,
            126, 157, 200, 31, 115, 93, 182, 131, 195, 255, 110, 199, 177, 80,
            63, 3, 87, 191, 225, 227, 65, 208, 28, 105, 254, 86, 84, 48, 153,
            86, 203, 234, 81, 104, 34, 251, 168, 166, 1, 116, 58, 1, 42, 120,
            150, 238, 141, 194,
        ];

        let ext_pubkey1 = bip32::ExtendedPubKey::from_str(
            "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ2\
            9ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8",
        )
        .unwrap();
        test_encoding_roundtrip(&ext_pubkey1, &EXT_PUBKEY1).unwrap();

        let ext_pubkey2 = bip32::ExtendedPubKey::from_str(
            "xpub6D4BDPcP2GT577Vvch3R8wDkScZWzQzMMUm3PWbmWvVJrZwQY4VUNgqFJP\
            MM3No2dFDFGTsxxpG5uJh7n7epu4trkrX7x7DogT5Uv6fcLW5",
        )
        .unwrap();
        test_encoding_roundtrip(&ext_pubkey2, &EXT_PUBKEY2).unwrap();
    }

    #[test]
    fn test_encoding_script() {
        static OP_RETURN: [u8; 40] = [
            0x26, 0x0, 0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed, 0x20, 0x28, 0xf,
            0x53, 0xf2, 0xd2, 0x16, 0x63, 0xca, 0xc8, 0x9e, 0x6b, 0xd2, 0xad,
            0x19, 0xed, 0xba, 0xbb, 0x4, 0x8c, 0xda, 0x8, 0xe7, 0x3e, 0xd1,
            0x9e, 0x92, 0x68, 0xd0, 0xaf, 0xea, 0x2a,
        ];
        static P2PK: [u8; 37] = [
            0x23, 0x0, 0x21, 0x2, 0x34, 0xe6, 0xa7, 0x9c, 0x53, 0x59, 0xc6,
            0x13, 0x76, 0x2d, 0x53, 0x7e, 0xe, 0x19, 0xd8, 0x6c, 0x77, 0xc1,
            0x66, 0x6d, 0x8c, 0x9a, 0xb0, 0x50, 0xf2, 0x3a, 0xcd, 0x19, 0x8e,
            0x97, 0xf9, 0x3e, 0xac,
        ];

        static P2PKH: [u8; 27] = [
            0x19, 0x0, 0x76, 0xa9, 0x14, 0xaa, 0xca, 0x99, 0x1e, 0x29, 0x8a,
            0xb8, 0x66, 0xab, 0x60, 0xff, 0x45, 0x22, 0x1b, 0x45, 0x8c, 0x70,
            0x33, 0x36, 0x5a, 0x88, 0xac,
        ];
        static P2SH: [u8; 25] = [
            0x17, 0x0, 0xa9, 0x14, 0x4d, 0xa3, 0x4a, 0xe8, 0x19, 0x9d, 0xbf,
            0x68, 0x4f, 0xe9, 0x7a, 0xf8, 0x70, 0x3f, 0x12, 0xe9, 0xf7, 0xaa,
            0xe6, 0x62, 0x87,
        ];
        static P2WPKH: [u8; 24] = [
            0x16, 0x0, 0x0, 0x14, 0xaa, 0xca, 0x99, 0x1e, 0x29, 0x8a, 0xb8,
            0x66, 0xab, 0x60, 0xff, 0x45, 0x22, 0x1b, 0x45, 0x8c, 0x70, 0x33,
            0x36, 0x5a,
        ];
        static P2WSH: [u8; 36] = [
            0x22, 0x0, 0x0, 0x20, 0x9d, 0x27, 0x71, 0x75, 0x73, 0x7f, 0xb5,
            0x0, 0x41, 0xe7, 0x5f, 0x64, 0x1a, 0xcf, 0x94, 0xd1, 0xd, 0xf9,
            0xb9, 0x72, 0x1d, 0xb8, 0xff, 0xfe, 0x87, 0x4a, 0xb5, 0x7f, 0x8f,
            0xfb, 0x6, 0x2e,
        ];

        // OP_RETURN
        let op_return: Script =
            test_vec_decoding_roundtrip(&OP_RETURN).unwrap();
        assert!(op_return.is_op_return());

        // P2PK
        let p2pk: Script = test_vec_decoding_roundtrip(&P2PK).unwrap();
        assert!(p2pk.is_p2pk());

        //P2PKH
        let p2pkh: Script = test_vec_decoding_roundtrip(&P2PKH).unwrap();
        assert!(p2pkh.is_p2pkh());

        //P2SH
        let p2sh: Script = test_vec_decoding_roundtrip(&P2SH).unwrap();
        assert!(p2sh.is_p2sh());

        //P2WPKH
        let p2wpkh: Script = test_vec_decoding_roundtrip(&P2WPKH).unwrap();
        assert!(p2wpkh.is_v0_p2wpkh());

        //P2WSH
        let p2wsh: Script = test_vec_decoding_roundtrip(&P2WSH).unwrap();
        assert!(p2wsh.is_v0_p2wsh());
    }
}
