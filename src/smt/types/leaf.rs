// Generated by Molecule 0.7.3

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(clippy::if_same_then_else)]

use ckb_types::prelude::*;
use molecule::{self, prelude::*};
extern crate alloc;
pub use alloc::vec::*;

use molecule::prelude::*;
#[derive(Clone)]
pub struct Byte32(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for Byte32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for Byte32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for Byte32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        let raw_data = hex_string(&self.raw_data());
        write!(f, "{}(0x{})", Self::NAME, raw_data)
    }
}
impl ::core::default::Default for Byte32 {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];
        Byte32::new_unchecked(v.into())
    }
}
impl Byte32 {
    pub const ITEM_COUNT: usize = 32;
    pub const ITEM_SIZE: usize = 1;
    pub const TOTAL_SIZE: usize = 32;

    pub fn nth0(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(0..1))
    }

    pub fn nth1(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(1..2))
    }

    pub fn nth2(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(2..3))
    }

    pub fn nth3(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(3..4))
    }

    pub fn nth4(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(4..5))
    }

    pub fn nth5(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(5..6))
    }

    pub fn nth6(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(6..7))
    }

    pub fn nth7(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(7..8))
    }

    pub fn nth8(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(8..9))
    }

    pub fn nth9(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(9..10))
    }

    pub fn nth10(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(10..11))
    }

    pub fn nth11(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(11..12))
    }

    pub fn nth12(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(12..13))
    }

    pub fn nth13(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(13..14))
    }

    pub fn nth14(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(14..15))
    }

    pub fn nth15(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(15..16))
    }

    pub fn nth16(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(16..17))
    }

    pub fn nth17(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(17..18))
    }

    pub fn nth18(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(18..19))
    }

    pub fn nth19(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(19..20))
    }

    pub fn nth20(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(20..21))
    }

    pub fn nth21(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(21..22))
    }

    pub fn nth22(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(22..23))
    }

    pub fn nth23(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(23..24))
    }

    pub fn nth24(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(24..25))
    }

    pub fn nth25(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(25..26))
    }

    pub fn nth26(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(26..27))
    }

    pub fn nth27(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(27..28))
    }

    pub fn nth28(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(28..29))
    }

    pub fn nth29(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(29..30))
    }

    pub fn nth30(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(30..31))
    }

    pub fn nth31(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(31..32))
    }

    pub fn raw_data(&self) -> molecule::bytes::Bytes {
        self.as_bytes()
    }

    pub fn as_reader<'r>(&'r self) -> Byte32Reader<'r> {
        Byte32Reader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for Byte32 {
    type Builder = Byte32Builder;

    const NAME: &'static str = "Byte32";

    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        Byte32(data)
    }

    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }

    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }

    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        Byte32Reader::from_slice(slice).map(|reader| reader.to_entity())
    }

    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        Byte32Reader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }

    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }

    fn as_builder(self) -> Self::Builder {
        Self::new_builder().set([
            self.nth0(),
            self.nth1(),
            self.nth2(),
            self.nth3(),
            self.nth4(),
            self.nth5(),
            self.nth6(),
            self.nth7(),
            self.nth8(),
            self.nth9(),
            self.nth10(),
            self.nth11(),
            self.nth12(),
            self.nth13(),
            self.nth14(),
            self.nth15(),
            self.nth16(),
            self.nth17(),
            self.nth18(),
            self.nth19(),
            self.nth20(),
            self.nth21(),
            self.nth22(),
            self.nth23(),
            self.nth24(),
            self.nth25(),
            self.nth26(),
            self.nth27(),
            self.nth28(),
            self.nth29(),
            self.nth30(),
            self.nth31(),
        ])
    }
}
#[derive(Clone, Copy)]
pub struct Byte32Reader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for Byte32Reader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for Byte32Reader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for Byte32Reader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        let raw_data = hex_string(&self.raw_data());
        write!(f, "{}(0x{})", Self::NAME, raw_data)
    }
}
impl<'r> Byte32Reader<'r> {
    pub const ITEM_COUNT: usize = 32;
    pub const ITEM_SIZE: usize = 1;
    pub const TOTAL_SIZE: usize = 32;

    pub fn nth0(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[0..1])
    }

    pub fn nth1(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[1..2])
    }

    pub fn nth2(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[2..3])
    }

    pub fn nth3(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[3..4])
    }

    pub fn nth4(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[4..5])
    }

    pub fn nth5(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[5..6])
    }

    pub fn nth6(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[6..7])
    }

    pub fn nth7(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[7..8])
    }

    pub fn nth8(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[8..9])
    }

    pub fn nth9(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[9..10])
    }

    pub fn nth10(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[10..11])
    }

    pub fn nth11(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[11..12])
    }

    pub fn nth12(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[12..13])
    }

    pub fn nth13(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[13..14])
    }

    pub fn nth14(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[14..15])
    }

    pub fn nth15(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[15..16])
    }

    pub fn nth16(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[16..17])
    }

    pub fn nth17(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[17..18])
    }

    pub fn nth18(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[18..19])
    }

    pub fn nth19(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[19..20])
    }

    pub fn nth20(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[20..21])
    }

    pub fn nth21(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[21..22])
    }

    pub fn nth22(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[22..23])
    }

    pub fn nth23(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[23..24])
    }

    pub fn nth24(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[24..25])
    }

    pub fn nth25(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[25..26])
    }

    pub fn nth26(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[26..27])
    }

    pub fn nth27(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[27..28])
    }

    pub fn nth28(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[28..29])
    }

    pub fn nth29(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[29..30])
    }

    pub fn nth30(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[30..31])
    }

    pub fn nth31(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[31..32])
    }

    pub fn raw_data(&self) -> &'r [u8] {
        self.as_slice()
    }
}
impl<'r> molecule::prelude::Reader<'r> for Byte32Reader<'r> {
    type Entity = Byte32;

    const NAME: &'static str = "Byte32Reader";

    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }

    fn new_unchecked(slice: &'r [u8]) -> Self {
        Byte32Reader(slice)
    }

    fn as_slice(&self) -> &'r [u8] {
        self.0
    }

    fn verify(slice: &[u8], _compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len != Self::TOTAL_SIZE {
            return ve!(Self, TotalSizeNotMatch, Self::TOTAL_SIZE, slice_len);
        }
        Ok(())
    }
}
pub struct Byte32Builder(pub(crate) [Byte; 32]);
impl ::core::fmt::Debug for Byte32Builder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:?})", Self::NAME, &self.0[..])
    }
}
impl ::core::default::Default for Byte32Builder {
    fn default() -> Self {
        Byte32Builder([
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
            Byte::default(),
        ])
    }
}
impl Byte32Builder {
    pub const ITEM_COUNT: usize = 32;
    pub const ITEM_SIZE: usize = 1;
    pub const TOTAL_SIZE: usize = 32;

    pub fn set(mut self, v: [Byte; 32]) -> Self {
        self.0 = v;
        self
    }

    pub fn nth0(mut self, v: Byte) -> Self {
        self.0[0] = v;
        self
    }

    pub fn nth1(mut self, v: Byte) -> Self {
        self.0[1] = v;
        self
    }

    pub fn nth2(mut self, v: Byte) -> Self {
        self.0[2] = v;
        self
    }

    pub fn nth3(mut self, v: Byte) -> Self {
        self.0[3] = v;
        self
    }

    pub fn nth4(mut self, v: Byte) -> Self {
        self.0[4] = v;
        self
    }

    pub fn nth5(mut self, v: Byte) -> Self {
        self.0[5] = v;
        self
    }

    pub fn nth6(mut self, v: Byte) -> Self {
        self.0[6] = v;
        self
    }

    pub fn nth7(mut self, v: Byte) -> Self {
        self.0[7] = v;
        self
    }

    pub fn nth8(mut self, v: Byte) -> Self {
        self.0[8] = v;
        self
    }

    pub fn nth9(mut self, v: Byte) -> Self {
        self.0[9] = v;
        self
    }

    pub fn nth10(mut self, v: Byte) -> Self {
        self.0[10] = v;
        self
    }

    pub fn nth11(mut self, v: Byte) -> Self {
        self.0[11] = v;
        self
    }

    pub fn nth12(mut self, v: Byte) -> Self {
        self.0[12] = v;
        self
    }

    pub fn nth13(mut self, v: Byte) -> Self {
        self.0[13] = v;
        self
    }

    pub fn nth14(mut self, v: Byte) -> Self {
        self.0[14] = v;
        self
    }

    pub fn nth15(mut self, v: Byte) -> Self {
        self.0[15] = v;
        self
    }

    pub fn nth16(mut self, v: Byte) -> Self {
        self.0[16] = v;
        self
    }

    pub fn nth17(mut self, v: Byte) -> Self {
        self.0[17] = v;
        self
    }

    pub fn nth18(mut self, v: Byte) -> Self {
        self.0[18] = v;
        self
    }

    pub fn nth19(mut self, v: Byte) -> Self {
        self.0[19] = v;
        self
    }

    pub fn nth20(mut self, v: Byte) -> Self {
        self.0[20] = v;
        self
    }

    pub fn nth21(mut self, v: Byte) -> Self {
        self.0[21] = v;
        self
    }

    pub fn nth22(mut self, v: Byte) -> Self {
        self.0[22] = v;
        self
    }

    pub fn nth23(mut self, v: Byte) -> Self {
        self.0[23] = v;
        self
    }

    pub fn nth24(mut self, v: Byte) -> Self {
        self.0[24] = v;
        self
    }

    pub fn nth25(mut self, v: Byte) -> Self {
        self.0[25] = v;
        self
    }

    pub fn nth26(mut self, v: Byte) -> Self {
        self.0[26] = v;
        self
    }

    pub fn nth27(mut self, v: Byte) -> Self {
        self.0[27] = v;
        self
    }

    pub fn nth28(mut self, v: Byte) -> Self {
        self.0[28] = v;
        self
    }

    pub fn nth29(mut self, v: Byte) -> Self {
        self.0[29] = v;
        self
    }

    pub fn nth30(mut self, v: Byte) -> Self {
        self.0[30] = v;
        self
    }

    pub fn nth31(mut self, v: Byte) -> Self {
        self.0[31] = v;
        self
    }
}
impl molecule::prelude::Builder for Byte32Builder {
    type Entity = Byte32;

    const NAME: &'static str = "Byte32Builder";

    fn expected_length(&self) -> usize {
        Self::TOTAL_SIZE
    }

    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        writer.write_all(self.0[0].as_slice())?;
        writer.write_all(self.0[1].as_slice())?;
        writer.write_all(self.0[2].as_slice())?;
        writer.write_all(self.0[3].as_slice())?;
        writer.write_all(self.0[4].as_slice())?;
        writer.write_all(self.0[5].as_slice())?;
        writer.write_all(self.0[6].as_slice())?;
        writer.write_all(self.0[7].as_slice())?;
        writer.write_all(self.0[8].as_slice())?;
        writer.write_all(self.0[9].as_slice())?;
        writer.write_all(self.0[10].as_slice())?;
        writer.write_all(self.0[11].as_slice())?;
        writer.write_all(self.0[12].as_slice())?;
        writer.write_all(self.0[13].as_slice())?;
        writer.write_all(self.0[14].as_slice())?;
        writer.write_all(self.0[15].as_slice())?;
        writer.write_all(self.0[16].as_slice())?;
        writer.write_all(self.0[17].as_slice())?;
        writer.write_all(self.0[18].as_slice())?;
        writer.write_all(self.0[19].as_slice())?;
        writer.write_all(self.0[20].as_slice())?;
        writer.write_all(self.0[21].as_slice())?;
        writer.write_all(self.0[22].as_slice())?;
        writer.write_all(self.0[23].as_slice())?;
        writer.write_all(self.0[24].as_slice())?;
        writer.write_all(self.0[25].as_slice())?;
        writer.write_all(self.0[26].as_slice())?;
        writer.write_all(self.0[27].as_slice())?;
        writer.write_all(self.0[28].as_slice())?;
        writer.write_all(self.0[29].as_slice())?;
        writer.write_all(self.0[30].as_slice())?;
        writer.write_all(self.0[31].as_slice())?;
        Ok(())
    }

    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        Byte32::new_unchecked(inner.into())
    }
}
#[derive(Clone)]
pub struct SMTLeafVec(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for SMTLeafVec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for SMTLeafVec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for SMTLeafVec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl ::core::default::Default for SMTLeafVec {
    fn default() -> Self {
        let v: Vec<u8> = vec![0, 0, 0, 0];
        SMTLeafVec::new_unchecked(v.into())
    }
}
impl SMTLeafVec {
    pub const ITEM_SIZE: usize = 64;

    pub fn total_size(&self) -> usize {
        molecule::NUMBER_SIZE + Self::ITEM_SIZE * self.item_count()
    }

    pub fn item_count(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }

    pub fn len(&self) -> usize {
        self.item_count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, idx: usize) -> Option<SMTLeaf> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }

    pub fn get_unchecked(&self, idx: usize) -> SMTLeaf {
        let start = molecule::NUMBER_SIZE + Self::ITEM_SIZE * idx;
        let end = start + Self::ITEM_SIZE;
        SMTLeaf::new_unchecked(self.0.slice(start..end))
    }

    pub fn as_reader<'r>(&'r self) -> SMTLeafVecReader<'r> {
        SMTLeafVecReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for SMTLeafVec {
    type Builder = SMTLeafVecBuilder;

    const NAME: &'static str = "SMTLeafVec";

    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        SMTLeafVec(data)
    }

    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }

    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }

    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        SMTLeafVecReader::from_slice(slice).map(|reader| reader.to_entity())
    }

    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        SMTLeafVecReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }

    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }

    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
#[derive(Clone, Copy)]
pub struct SMTLeafVecReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for SMTLeafVecReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for SMTLeafVecReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for SMTLeafVecReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl<'r> SMTLeafVecReader<'r> {
    pub const ITEM_SIZE: usize = 64;

    pub fn total_size(&self) -> usize {
        molecule::NUMBER_SIZE + Self::ITEM_SIZE * self.item_count()
    }

    pub fn item_count(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }

    pub fn len(&self) -> usize {
        self.item_count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, idx: usize) -> Option<SMTLeafReader<'r>> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }

    pub fn get_unchecked(&self, idx: usize) -> SMTLeafReader<'r> {
        let start = molecule::NUMBER_SIZE + Self::ITEM_SIZE * idx;
        let end = start + Self::ITEM_SIZE;
        SMTLeafReader::new_unchecked(&self.as_slice()[start..end])
    }
}
impl<'r> molecule::prelude::Reader<'r> for SMTLeafVecReader<'r> {
    type Entity = SMTLeafVec;

    const NAME: &'static str = "SMTLeafVecReader";

    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }

    fn new_unchecked(slice: &'r [u8]) -> Self {
        SMTLeafVecReader(slice)
    }

    fn as_slice(&self) -> &'r [u8] {
        self.0
    }

    fn verify(slice: &[u8], _compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let item_count = molecule::unpack_number(slice) as usize;
        if item_count == 0 {
            if slice_len != molecule::NUMBER_SIZE {
                return ve!(Self, TotalSizeNotMatch, molecule::NUMBER_SIZE, slice_len);
            }
            return Ok(());
        }
        let total_size = molecule::NUMBER_SIZE + Self::ITEM_SIZE * item_count;
        if slice_len != total_size {
            return ve!(Self, TotalSizeNotMatch, total_size, slice_len);
        }
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct SMTLeafVecBuilder(pub(crate) Vec<SMTLeaf>);
impl SMTLeafVecBuilder {
    pub const ITEM_SIZE: usize = 64;

    pub fn set(mut self, v: Vec<SMTLeaf>) -> Self {
        self.0 = v;
        self
    }

    pub fn push(mut self, v: SMTLeaf) -> Self {
        self.0.push(v);
        self
    }

    pub fn extend<T: ::core::iter::IntoIterator<Item = SMTLeaf>>(mut self, iter: T) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }

    pub fn replace(&mut self, index: usize, v: SMTLeaf) -> Option<SMTLeaf> {
        self.0
            .get_mut(index)
            .map(|item| ::core::mem::replace(item, v))
    }
}
impl molecule::prelude::Builder for SMTLeafVecBuilder {
    type Entity = SMTLeafVec;

    const NAME: &'static str = "SMTLeafVecBuilder";

    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE + Self::ITEM_SIZE * self.0.len()
    }

    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        writer.write_all(&molecule::pack_number(self.0.len() as molecule::Number))?;
        for inner in &self.0[..] {
            writer.write_all(inner.as_slice())?;
        }
        Ok(())
    }

    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        SMTLeafVec::new_unchecked(inner.into())
    }
}
pub struct SMTLeafVecIterator(SMTLeafVec, usize, usize);
impl ::core::iter::Iterator for SMTLeafVecIterator {
    type Item = SMTLeaf;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::core::iter::ExactSizeIterator for SMTLeafVecIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::core::iter::IntoIterator for SMTLeafVec {
    type IntoIter = SMTLeafVecIterator;
    type Item = SMTLeaf;

    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        SMTLeafVecIterator(self, 0, len)
    }
}
impl<'r> SMTLeafVecReader<'r> {
    pub fn iter<'t>(&'t self) -> SMTLeafVecReaderIterator<'t, 'r> {
        SMTLeafVecReaderIterator(&self, 0, self.len())
    }
}
pub struct SMTLeafVecReaderIterator<'t, 'r>(&'t SMTLeafVecReader<'r>, usize, usize);
impl<'t: 'r, 'r> ::core::iter::Iterator for SMTLeafVecReaderIterator<'t, 'r> {
    type Item = SMTLeafReader<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl<'t: 'r, 'r> ::core::iter::ExactSizeIterator for SMTLeafVecReaderIterator<'t, 'r> {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
#[derive(Clone)]
pub struct SMTLeaf(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for SMTLeaf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for SMTLeaf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for SMTLeaf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "key", self.key())?;
        write!(f, ", {}: {}", "value", self.value())?;
        write!(f, " }}")
    }
}
impl ::core::default::Default for SMTLeaf {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];
        SMTLeaf::new_unchecked(v.into())
    }
}
impl SMTLeaf {
    pub const FIELD_COUNT: usize = 2;
    pub const FIELD_SIZES: [usize; 2] = [32, 32];
    pub const TOTAL_SIZE: usize = 64;

    pub fn key(&self) -> Byte32 {
        Byte32::new_unchecked(self.0.slice(0..32))
    }

    pub fn value(&self) -> Byte32 {
        Byte32::new_unchecked(self.0.slice(32..64))
    }

    pub fn as_reader<'r>(&'r self) -> SMTLeafReader<'r> {
        SMTLeafReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for SMTLeaf {
    type Builder = SMTLeafBuilder;

    const NAME: &'static str = "SMTLeaf";

    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        SMTLeaf(data)
    }

    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }

    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }

    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        SMTLeafReader::from_slice(slice).map(|reader| reader.to_entity())
    }

    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        SMTLeafReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }

    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }

    fn as_builder(self) -> Self::Builder {
        Self::new_builder().key(self.key()).value(self.value())
    }
}
#[derive(Clone, Copy)]
pub struct SMTLeafReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for SMTLeafReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for SMTLeafReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for SMTLeafReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "key", self.key())?;
        write!(f, ", {}: {}", "value", self.value())?;
        write!(f, " }}")
    }
}
impl<'r> SMTLeafReader<'r> {
    pub const FIELD_COUNT: usize = 2;
    pub const FIELD_SIZES: [usize; 2] = [32, 32];
    pub const TOTAL_SIZE: usize = 64;

    pub fn key(&self) -> Byte32Reader<'r> {
        Byte32Reader::new_unchecked(&self.as_slice()[0..32])
    }

    pub fn value(&self) -> Byte32Reader<'r> {
        Byte32Reader::new_unchecked(&self.as_slice()[32..64])
    }
}
impl<'r> molecule::prelude::Reader<'r> for SMTLeafReader<'r> {
    type Entity = SMTLeaf;

    const NAME: &'static str = "SMTLeafReader";

    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }

    fn new_unchecked(slice: &'r [u8]) -> Self {
        SMTLeafReader(slice)
    }

    fn as_slice(&self) -> &'r [u8] {
        self.0
    }

    fn verify(slice: &[u8], _compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len != Self::TOTAL_SIZE {
            return ve!(Self, TotalSizeNotMatch, Self::TOTAL_SIZE, slice_len);
        }
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct SMTLeafBuilder {
    pub(crate) key:   Byte32,
    pub(crate) value: Byte32,
}
impl SMTLeafBuilder {
    pub const FIELD_COUNT: usize = 2;
    pub const FIELD_SIZES: [usize; 2] = [32, 32];
    pub const TOTAL_SIZE: usize = 64;

    pub fn key(mut self, v: Byte32) -> Self {
        self.key = v;
        self
    }

    pub fn value(mut self, v: Byte32) -> Self {
        self.value = v;
        self
    }
}
impl molecule::prelude::Builder for SMTLeafBuilder {
    type Entity = SMTLeaf;

    const NAME: &'static str = "SMTLeafBuilder";

    fn expected_length(&self) -> usize {
        Self::TOTAL_SIZE
    }

    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        writer.write_all(self.key.as_slice())?;
        writer.write_all(self.value.as_slice())?;
        Ok(())
    }

    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        SMTLeaf::new_unchecked(inner.into())
    }
}
