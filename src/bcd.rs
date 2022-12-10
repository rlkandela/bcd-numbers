use crate::BCDConversionError;
use std::fmt::{Debug, Display};

#[derive(PartialEq, Clone, Copy)]
pub struct BCD<const BYTES: usize> {
    data: [u8;BYTES]
}

#[derive(PartialEq, Clone)]
pub struct DynBCD {
    data: Vec<u8>
}

pub trait Convertible<T> {
    fn convert(&self) -> T;
}

fn check_invalid_byte(val: u8) -> bool {
    let high = (val & 0xf0) >> 0x04;
    let low = val & 0x0f;
    low > 9 || high > 9
}

fn check_invalid_bytes<'a>(it: impl Iterator<Item = &'a u8>) -> bool {
    let mut it = it;
    it.any(|val| check_invalid_byte(val.to_owned()))
}

impl<const BYTES_OG: usize, const BYTES_DST: usize> Convertible<BCD<BYTES_DST>> for BCD<BYTES_OG> {
    fn convert(&self) -> BCD<BYTES_DST> {
        if BYTES_OG >= BYTES_DST {
            BCD {
                data: self.data.into_iter().rev().take(BYTES_DST).rev().collect::<Vec<u8>>().try_into().unwrap()
            }
        }
        else {
            let mut new_data = vec![0;BYTES_DST-BYTES_OG];
            new_data.append(&mut Vec::from(self.data));
            BCD {
                data: new_data.try_into().unwrap()
            }
        }
    }
}

impl<const BYTES: usize> Debug for BCD<BYTES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BCD")
            .field("data", &self.data)
            .field("computed_value", &Into::<u128>::into((*self).convert()))
            .finish()
    }
}

impl<const BYTES: usize> Display for BCD<BYTES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BCD<{}> {{ \"computed_value\": {} }}", BYTES, Into::<u128>::into((*self).convert()))
    }
}

impl Debug for DynBCD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynBCD")
            .field("data", &self.data)
            .field("computed_value", &Into::<u128>::into(self.clone()))
            .finish()
    }
}

impl Display for DynBCD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DynBCD {{ \"computed_value\": {} }}", Into::<u128>::into(self.clone()))
    }
}

impl<const BYTES: usize> IntoIterator for BCD<BYTES> {
    type Item = u8;

    type IntoIter = std::array::IntoIter<Self::Item,BYTES>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl IntoIterator for DynBCD {
    type Item = u8;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<const BYTES: usize> From<BCD<BYTES>> for Vec<u8> {
    fn from(data: BCD<BYTES>) -> Self {
        Vec::from(data.data)
    }
}

impl From<DynBCD> for Vec<u8> {
    fn from(data: DynBCD) -> Self {
        data.data
    }
}

impl<const BYTES: usize> From<DynBCD> for BCD<BYTES> {
    fn from(value: DynBCD) -> Self {
        let mut new_data: Vec<u8> = value.data.into_iter().rev().take(BYTES).collect();
        while new_data.len() < BYTES {
            new_data.push(0);
        }
        let new_data: Vec<u8> = new_data.into_iter().rev().collect();
        Self {
            data: new_data.try_into().unwrap()
        }
    }
}

impl<const BYTES: usize> From<BCD<BYTES>> for DynBCD {
    fn from(value: BCD<BYTES>) -> Self {
        Self {
            data: Vec::from(value.data)
        }
    }
}

impl TryFrom<&[u8]> for DynBCD {
    type Error = BCDConversionError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if check_invalid_bytes(value.iter()) {
            return Err(BCDConversionError::new("Invalid format, found A-F"));
        }
        Ok(Self {
            data: Vec::from(value)
        })
    }
}

impl<const BYTES: usize> TryFrom<&[u8]> for BCD<BYTES> {
    type Error = BCDConversionError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if check_invalid_bytes(value.iter()) {
            return Err(BCDConversionError::new("Invalid format, found A-F"));
        }
        if value.len() >= BYTES {
            let buffer: Vec<u8> = value.iter()
                .rev().take(BYTES).rev()
                .copied()
                .collect();
            Ok(Self {
                data: buffer.try_into().unwrap()
            })
        }
        else {
            let mut buffer = vec![0;BYTES - value.len()];
            buffer.extend(value.iter().copied());
            Ok(Self {
                data: buffer.try_into().unwrap()
            })
        }
    }
}

impl<const BYTES_OG: usize, const BYTES_DST: usize> TryFrom<[u8;BYTES_OG]> for BCD<BYTES_DST> {
    type Error = BCDConversionError;

    fn try_from(value: [u8;BYTES_OG]) -> Result<Self, Self::Error> {
        if check_invalid_bytes(value.iter()) {
            return Err(BCDConversionError::new("Invalid format. Found A-F"));
        }
        if BYTES_OG >= BYTES_DST {
            Ok(Self {
                data: value[BYTES_OG-BYTES_DST..].try_into().unwrap()
            })
        }
        else {
            let mut new_data = vec![0;BYTES_DST-BYTES_OG];
            new_data.append(&mut Vec::from(value));
            Ok(Self {
                data: new_data.try_into().unwrap()
            })
        }
    }
}

impl<const BYTES_OG: usize, const BYTES_DST: usize> From<BCD<BYTES_OG>> for [u8;BYTES_DST] {
    fn from(data: BCD<BYTES_OG>) -> Self {
        if BYTES_OG >= BYTES_DST {
            data.data.into_iter().rev().take(BYTES_DST).rev().collect::<Vec<u8>>().try_into().unwrap()
        }
        else {
            let mut new_data = vec![0;BYTES_DST-BYTES_OG];
            new_data.append(&mut Vec::from(data.data));
            new_data.try_into().unwrap()
        }
    }
}

impl TryFrom<u8> for BCD<1> {
    type Error = BCDConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let n_bytes = value.to_ne_bytes().into_iter().count();
        let max_val = (0..n_bytes).into_iter().fold(0, |acc, _| acc*100+99);
        if value > max_val { return Err(Self::Error::new_with_template_description("u8", value, max_val)); }
        let high = value/10;
        let low = value%10;
        Ok(Self{data: [high<<4|low]})
    }
}

impl From<BCD<1>> for u8 {
    fn from(val: BCD<1>) -> Self {
        let high = (val.data[0] & 0xf0) >> 4;
        let low  = val.data[0] & 0x0f;
        high*10+low
    }
}

impl TryFrom<u16> for BCD<2> {
    type Error = BCDConversionError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let n_bytes = value.to_ne_bytes().into_iter().count();
        let max_val = (0..n_bytes).into_iter().fold(0, |acc, _| acc*100+99);
        if value > max_val { return Err(Self::Error::new_with_template_description("u16", value, max_val)); }
        let mut value = value;
        let mut buffer: Vec<BCD<1>> = vec![];
        while value != 0 {
            let current = value%100;
            buffer.push((current as u8).try_into()?);
            value /= 100;
        }
        while buffer.len() < n_bytes {
            buffer.push(BCD{ data: [0] });
        }
        let buffer: [u8; 2] = buffer.into_iter()
            .rev()
            .flat_map(|item| item.data)
            .collect::<Vec<u8>>()
            .try_into().unwrap();
        Ok(Self::try_from(buffer).unwrap())
    }
}

impl From<BCD<2>> for u16 {
    fn from(value: BCD<2>) -> Self {
        value.data.into_iter()
            .fold(0, |acc, x| acc*100 + (Into::<u8>::into(BCD::try_from([x]).unwrap()) as Self))
    }
}

impl TryFrom<u32> for BCD<4> {
    type Error = BCDConversionError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let n_bytes = value.to_ne_bytes().into_iter().count();
        let max_val = (0..n_bytes).into_iter().fold(0, |acc, _| acc*100+99);
        if value > max_val { return Err(Self::Error::new_with_template_description("u32", value, max_val)); }
        let mut value = value;
        let mut buffer: Vec<BCD<1>> = vec![];
        while value != 0 {
            let current = value%100;
            buffer.push((current as u8).try_into()?);
            value /= 100;
        }
        while buffer.len() < n_bytes {
            buffer.push(BCD{ data: [0] });
        }
        let buffer: [u8; 4] = buffer.into_iter()
            .rev()
            .flat_map(|item| item.data)
            .collect::<Vec<u8>>()
            .try_into().unwrap();
        Ok(Self::try_from(buffer).unwrap())
    }
}

impl From<BCD<4>> for u32 {
    fn from(value: BCD<4>) -> Self {
        value.data.into_iter()
            .fold(0, |acc, x| acc*100 + (Into::<u8>::into(BCD::try_from([x]).unwrap()) as Self))
    }
}

impl TryFrom<u64> for BCD<8> {
    type Error = BCDConversionError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let n_bytes = value.to_ne_bytes().into_iter().count();
        let max_val = (0..n_bytes).into_iter().fold(0, |acc, _| acc*100+99);
        if value > max_val { return Err(Self::Error::new_with_template_description("u32", value, max_val)); }
        let mut value = value;
        let mut buffer: Vec<BCD<1>> = vec![];
        while value != 0 {
            let current = value%100;
            buffer.push((current as u8).try_into()?);
            value /= 100;
        }
        while buffer.len() < n_bytes {
            buffer.push(BCD{ data: [0] });
        }
        let buffer: [u8; 8] = buffer.into_iter()
            .rev()
            .flat_map(|item| item.data)
            .collect::<Vec<u8>>()
            .try_into().unwrap();
        Ok(Self::try_from(buffer).unwrap())
    }
}

impl From<BCD<8>> for u64 {
    fn from(value: BCD<8>) -> Self {
        value.data.into_iter()
            .fold(0, |acc, x| acc*100 + (Into::<u8>::into(BCD::try_from([x]).unwrap()) as Self))
    }
}

impl TryFrom<u128> for BCD<16> {
    type Error = BCDConversionError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        let n_bytes = value.to_ne_bytes().into_iter().count();
        let max_val = (0..n_bytes).into_iter().fold(0, |acc, _| acc*100+99);
        if value > max_val { return Err(Self::Error::new_with_template_description("u32", value, max_val)); }
        let mut value = value;
        let mut buffer: Vec<BCD<1>> = vec![];
        while value != 0 {
            let current = value%100;
            buffer.push((current as u8).try_into()?);
            value /= 100;
        }
        while buffer.len() < n_bytes {
            buffer.push(BCD{ data: [0] });
        }
        let buffer: [u8; 16] = buffer.into_iter()
            .rev()
            .flat_map(|item| item.data)
            .collect::<Vec<u8>>()
            .try_into().unwrap();
        Ok(Self::try_from(buffer).unwrap())
    }
}

impl From<BCD<16>> for u128 {
    fn from(value: BCD<16>) -> Self {
        value.data.into_iter()
            .fold(0, |acc, x| acc*100 + (Into::<u8>::into(BCD::try_from([x]).unwrap()) as Self))
    }
}

impl TryFrom<u8> for DynBCD {
    type Error = BCDConversionError;

    fn try_from(value: u8) -> Result<DynBCD, Self::Error> {
        let n_bytes = value.to_ne_bytes().into_iter().count();
        let max_val = (0..n_bytes).into_iter().fold(0, |acc, _| acc*100+99);
        if value > max_val { return Err(Self::Error::new_with_template_description("u8", value, max_val)); }
        let low = value % 10;
        let high = value / 10;
        Ok(Self {
            data: vec![high<<4|low]
        })
    }
}

impl From<DynBCD> for u8 {
    fn from(value: DynBCD) -> Self {
        let value = value.data.into_iter().rev().next().unwrap();
        let high = (value & 0xf0) >> 4;
        let low = value & 0x0f;
        high*10+low
    }
}

impl TryFrom<u16> for DynBCD {
    type Error = BCDConversionError;
    fn try_from(value: u16) -> Result<DynBCD, Self::Error> {
        let n_bytes = value.to_ne_bytes().into_iter().count();
        let max_val = (0..n_bytes).into_iter().fold(0, |acc, _| acc*100+99);
        if value > max_val { return Err(Self::Error::new_with_template_description("u16", value, max_val)); }
        let mut value = value;
        let mut buffer: Vec<BCD<1>> = vec![];
        while value != 0 {
            let current = (value % 100) as u8;
            value /= 100;
            buffer.push(current.try_into().unwrap());
        }
        let buffer = buffer.into_iter().rev().flat_map(|item| item.data).collect();
        Ok(Self {
            data: buffer
        })
    }
}

impl From<DynBCD> for u16 {
    fn from(value: DynBCD) -> Self {
        value.data.into_iter()
            .fold(0, |acc, x| acc*100 + (Into::<u8>::into(BCD::try_from([x]).unwrap()) as Self))
    }
}

impl TryFrom<u32> for DynBCD {
    type Error = BCDConversionError;

    fn try_from(value: u32) -> Result<DynBCD, Self::Error> {
        let n_bytes = value.to_ne_bytes().into_iter().count();
        let max_val = (0..n_bytes).into_iter().fold(0, |acc, _| acc*100+99);
        if value > max_val { return Err(Self::Error::new_with_template_description("u32", value, max_val)); }
        let mut value = value;
        let mut buffer: Vec<BCD<1>> = vec![];
        while value != 0 {
            let current = (value % 100) as u8;
            value /= 100;
            buffer.push(current.try_into().unwrap());
        }
        let buffer = buffer.into_iter().rev().flat_map(|item| item.data).collect();
        Ok(Self {
            data: buffer
        })
    }
}

impl From<DynBCD> for u32 {
    fn from(value: DynBCD) -> Self {
        value.data.into_iter()
            .fold(0, |acc, x| acc*100 + (Into::<u8>::into(BCD::try_from([x]).unwrap()) as Self))
    }
}

impl TryFrom<u64> for DynBCD {
    type Error = BCDConversionError;

    fn try_from(value: u64) -> Result<DynBCD, Self::Error> {
        let n_bytes = value.to_ne_bytes().into_iter().count();
        let max_val = (0..n_bytes).into_iter().fold(0, |acc, _| acc*100+99);
        if value > max_val { return Err(Self::Error::new_with_template_description("u64", value, max_val)); }
        let mut value = value;
        let mut buffer: Vec<BCD<1>> = vec![];
        while value != 0 {
            let current = (value % 100) as u8;
            value /= 100;
            buffer.push(current.try_into().unwrap());
        }
        let buffer = buffer.into_iter().rev().flat_map(|item| item.data).collect();
        Ok(Self {
            data: buffer
        })
    }
}

impl From<DynBCD> for u64 {
    fn from(value: DynBCD) -> Self {
        value.data.into_iter()
            .fold(0, |acc, x| acc*100 + (Into::<u8>::into(BCD::try_from([x]).unwrap()) as Self))
    }
}

impl TryFrom<u128> for DynBCD {
    type Error = BCDConversionError;

    fn try_from(value: u128) -> Result<DynBCD, Self::Error> {
        let n_bytes = value.to_ne_bytes().into_iter().count();
        let max_val = (0..n_bytes).into_iter().fold(0, |acc, _| acc*100+99);
        if value > max_val { return Err(Self::Error::new_with_template_description("u128", value, max_val)); }
        let mut value = value;
        let mut buffer: Vec<BCD<1>> = vec![];
        while value != 0 {
            let current = (value % 100) as u8;
            value /= 100;
            buffer.push(current.try_into().unwrap());
        }
        let buffer = buffer.into_iter().rev().flat_map(|item| item.data).collect();
        Ok(Self {
            data: buffer
        })
    }
}

impl From<DynBCD> for u128 {
    fn from(value: DynBCD) -> Self {
        value.data.into_iter()
            .fold(0, |acc, x| acc*100 + (Into::<u8>::into(BCD::try_from([x]).unwrap()) as Self))
    }
}

impl<const BYTES: usize> BCD<BYTES> {
    pub fn new(value: u128) -> Self {
        let new_val: BCD<16> = value.try_into().unwrap();
        new_val.convert()
    }

    pub fn get_number(&self) -> u128 {
        (*self).convert().into()
    }
}

impl DynBCD {
    pub fn new(value: u128) -> Self {
        value.try_into().unwrap()
    }

    pub fn get_number(&self) -> u128 {
        self.clone().into()
    }
}
