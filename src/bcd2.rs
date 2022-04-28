use crate::BCDConversionError;

#[derive(Debug)]
pub struct BCD {
    data: Vec<u8>
}

impl<const S: usize> From<[u8;S]> for BCD {
    fn from(data: [u8;S]) -> Self {
        Self {
            data: Vec::from(data)
        }
    }
}

impl<const S: usize> From<BCD> for [u8;S] {
    fn from(val: BCD) -> Self {
        //val.data[val.data.len()-S..].try_into().unwrap()
        let len = val.data.len();
        if len >= S {
            val.data[len-S..].try_into().unwrap()
        }
        else {
            let mut val = val;
            let mut ret = vec![0;S-len];
            ret.append(&mut val.data);
            ret.try_into().unwrap()
        }
    }
}

impl TryFrom<u8> for BCD {
    type Error = BCDConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let max_val = value.to_ne_bytes().into_iter().fold(0, |acc, _| acc*10+9);
        if value > max_val { return Err(Self::Error::new_with_template_description("u8", value, max_val)); }
        let high = value / 10;
        let low = value % 10;
        let byte_val = high<<4 | low;
        Ok(Self {data: vec![byte_val]})
    }
}

impl From<BCD> for u8 {
    fn from(val: BCD) -> Self {
        let byte_val = val.data[0];
        let high = (byte_val & 0xf0) >> 4;
        let low = byte_val & 0x0f;
        high*10 + low
    }
}

impl TryFrom<u16> for BCD {
    type Error = BCDConversionError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let n_bytes = value.to_ne_bytes().into_iter().count();
        let max_val = vec![();n_bytes].into_iter().fold(0, |acc, _| acc*10+9);
        if value > max_val { return Err(Self::Error::new_with_template_description("u16", value, max_val)); }
        let mut value = value;
        let mut buffer = vec![];
        while value != 0 {
            let low = (value % 100) as u8;
            value = value / 100;
            buffer.push(BCD::try_from(low)?);
        }
        
        while buffer.len() < n_bytes {
            buffer.push(BCD::from([0]));
        }
        let buffer = buffer.into_iter()
            .rev()
            .map(|item| item.data)
            .flatten()
            .collect();

        Ok(Self {
            data: buffer
        })
    }
}

impl From<BCD> for u16 {
    fn from(val: BCD) -> Self {
        let n_bytes = Self::to_ne_bytes(0).len();
        val.data.into_iter()
            .rev().take(n_bytes).rev()    // Al ser big endian, queremos coger los n bytes del final
            .fold(0, |acc, x| acc*100 + (x as Self))
    }
}

impl TryFrom<u32> for BCD {
    type Error = BCDConversionError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let n_bytes = value.to_ne_bytes().into_iter().count();
        let max_val = vec![();n_bytes].into_iter().fold(0, |acc, _| acc*10+9);
        if value > max_val { return Err(Self::Error::new_with_template_description("u32", value, max_val)); }
        let mut value = value;
        let mut buffer = vec![];
        while value != 0 {
            let low = (value % 100) as u8;
            value = value / 100;
            buffer.push(BCD::try_from(low)?);
        }

        while buffer.len() < n_bytes {
            buffer.push(BCD::from([0]));
        }
        let buffer = buffer.into_iter()
            .rev()
            .map(|item| item.data)
            .flatten()
            .collect();

        Ok(Self {
            data: buffer
        })
    }
}

impl From<BCD> for u32 {
    fn from(val: BCD) -> Self {
        let n_bytes = Self::to_ne_bytes(0).len();
        val.data.into_iter()
            .rev().take(n_bytes).rev()    // Al ser big endian, queremos coger los n bytes del final
            .fold(0, |acc, x| acc*100 + (x as Self))
    }
}

impl TryFrom<u64> for BCD {
    type Error = BCDConversionError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let n_bytes = value.to_ne_bytes().into_iter().count();
        let max_val = vec![();n_bytes].into_iter().fold(0, |acc, _| acc*10+9);
        if value > max_val { return Err(Self::Error::new_with_template_description("u32", value, max_val)); }
        let mut value = value;
        let mut buffer = vec![];
        while value != 0 {
            let low = (value % 100) as u8;
            value = value / 100;
            buffer.push(BCD::try_from(low)?);
        }

        while buffer.len() < n_bytes {
            buffer.push(BCD::from([0]));
        }
        let buffer = buffer.into_iter()
            .rev()
            .map(|item| item.data)
            .flatten()
            .collect();

        Ok(Self {
            data: buffer
        })
    }
}

impl From<BCD> for u64 {
    fn from(val: BCD) -> Self {
        let n_bytes = Self::to_ne_bytes(0).len();
        val.data.into_iter()
            .rev().take(n_bytes).rev()    // Al ser big endian, queremos coger los n bytes del final
            .fold(0, |acc, x| acc*100 + (x as Self))
    }
}

impl TryFrom<u128> for BCD {
    type Error = BCDConversionError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        let n_bytes = value.to_ne_bytes().into_iter().count();
        let max_val = vec![();n_bytes].into_iter().fold(0, |acc, _| acc*10+9);
        if value > max_val { return Err(Self::Error::new_with_template_description("u32", value, max_val)); }
        let mut value = value;
        let mut buffer = vec![];
        while value != 0 {
            let low = (value % 100) as u8;
            value = value / 100;
            buffer.push(BCD::try_from(low)?);
        }

        while buffer.len() < n_bytes {
            buffer.push(BCD::from([0]));
        }
        let buffer = buffer.into_iter()
            .rev()
            .map(|item| item.data)
            .flatten()
            .collect();

        Ok(Self {
            data: buffer
        })
    }
}

impl From<BCD> for u128 {
    fn from(val: BCD) -> Self {
        let n_bytes = Self::to_ne_bytes(0).len();
        val.data.into_iter()
            .rev().take(n_bytes).rev()    // Al ser big endian, queremos coger los n bytes del final
            .fold(0, |acc, x| acc*100 + (x as Self))
    }
}

impl TryFrom<usize> for BCD {
    type Error = BCDConversionError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let n_bytes = value.to_ne_bytes().into_iter().count();
        let max_val = vec![();n_bytes].into_iter().fold(0, |acc, _| acc*10+9);
        if value > max_val { return Err(Self::Error::new_with_template_description("u32", value, max_val)); }
        let mut value = value;
        let mut buffer = vec![];
        while value != 0 {
            let low = (value % 100) as u8;
            value = value / 100;
            buffer.push(BCD::try_from(low)?);
        }

        while buffer.len() < n_bytes {
            buffer.push(BCD::from([0]));
        }
        let buffer = buffer.into_iter()
            .rev()
            .map(|item| item.data)
            .flatten()
            .collect();

        Ok(Self {
            data: buffer
        })
    }
}

impl From<BCD> for usize {
    fn from(val: BCD) -> Self {
        let n_bytes = Self::to_ne_bytes(0).len();
        val.data.into_iter()
            .rev().take(n_bytes).rev()    // Al ser big endian, queremos coger los n bytes del final
            .fold(0, |acc, x| acc*100 + (x as Self))
    }
}
