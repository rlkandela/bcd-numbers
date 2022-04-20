
#[derive(Debug)]
pub struct BCDConversionError {
    description: String
}

impl BCDConversionError {
    pub fn new(description: String) -> Self {
        Self { description }
    }

    pub fn new_boxed(description: String) -> Box<Self> {
        Box::new(Self::new(description))
    }

    pub fn new_with_template_description<T>(t: &str, v: T, max: &str) -> Self 
        where T: std::fmt::Display
    {
        Self::new(format!("Error on {} to bcd, passed in value ({}) exceeds maximum of {}", t, v,max))
    }

    pub fn description<'self_lt>(&'self_lt self) -> &'self_lt str {
        &self.description
    }
}

impl std::fmt::Display for BCDConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for BCDConversionError {}

pub trait ToBCD : Sized {
    fn to_bcd(self) -> Result<Self, BCDConversionError>;
}

pub trait FromBCD : Sized {
    fn from_bcd(self) -> Self;
}

impl ToBCD for u8 {
    fn to_bcd(self) -> Result<Self, BCDConversionError> {
        if self > 99 { return Err(BCDConversionError::new_with_template_description("u8", self, "99")); }
        let d1 = self%10;
        let d2 = self/10;
        Ok((d2 << 4) | d1)
    }
}

impl FromBCD for u8 {
    fn from_bcd(self) -> Self {
        let d1 = self&0x0f;
        let d2 = (self&0xf0) >> 4;
        d2*10 + d1
    }
}

impl ToBCD for u16 {
    fn to_bcd(self) -> Result<Self, BCDConversionError> {
        if self > 9999 { return Err(BCDConversionError::new_with_template_description("u16", self, "9999")); }
        let mut value = self;
        let mut result = 0;
        let mut shift = 0;
        while value != 0 {
            let lower = (value%100) as u8;
            let lower = lower.to_bcd().unwrap() as Self;
            result |= lower << shift;
            value /= 100;
            shift += 8;
        }
        Ok(result)
    }
}

impl FromBCD for u16 {
    fn from_bcd(self) -> Self {
        let mut value = self;
        let mut result = 0;
        let mut digit_base = 1;
        while value != 0 {
            let lower = (value & 0xff) as u8;
            let lower = lower.from_bcd() as Self;
            result += lower*digit_base;
            value >>= 8;
            digit_base *= 100;
        }
        result
    }
}

impl ToBCD for u32 {
    fn to_bcd(self) -> Result<Self, BCDConversionError> {
        if self > 99999999 { return Err(BCDConversionError::new_with_template_description("u32", self, "99999999")); }
        let mut value = self;
        let mut result = 0;
        let mut shift = 0;
        while value != 0 {
            let lower = (value%100) as u8;
            let lower = lower.to_bcd().unwrap() as Self;
            result |= lower << shift;
            value /= 100;
            shift += 8;
        }
        Ok(result)
    }
}

impl FromBCD for u32 {
    fn from_bcd(self) -> Self {
        let mut value = self;
        let mut result = 0;
        let mut digit_base = 1;
        while value != 0 {
            let lower = (value & 0xff) as u8;
            let lower = lower.from_bcd() as Self;
            result += lower*digit_base;
            value >>= 8;
            digit_base *= 100;
        }
        result
    }
}

impl ToBCD for u64 {
    fn to_bcd(self) -> Result<Self, BCDConversionError> {
        if self > 9999999999999999 { return Err(BCDConversionError::new_with_template_description("u64", self, "9999999999999999")); }
        let mut value = self;
        let mut result = 0;
        let mut shift = 0;
        while value != 0 {
            let lower = (value%100) as u8;
            let lower = lower.to_bcd().unwrap() as Self;
            result |= lower << shift;
            value /= 100;
            shift += 8;
        }
        Ok(result)
    }
}

impl FromBCD for u64 {
    fn from_bcd(self) -> Self {
        let mut value = self;
        let mut result = 0;
        let mut digit_base = 1;
        while value != 0 {
            let lower = (value & 0xff) as u8;
            let lower = lower.from_bcd() as Self;
            result += lower*digit_base;
            value >>= 8;
            digit_base *= 100;
        }
        result
    }
}

impl ToBCD for u128 {
    fn to_bcd(self) -> Result<Self, BCDConversionError> {
        if self > 99999999999999999999999999999999 { return Err(BCDConversionError::new_with_template_description("u128", self, "99999999999999999999999999999999")); }
        let mut value = self;
        let mut result = 0;
        let mut shift = 0;
        while value != 0 {
            let lower = (value%100) as u8;
            let lower = lower.to_bcd().unwrap() as Self;
            result |= lower << shift;
            value /= 100;
            shift += 8;
        }
        Ok(result)
    }
}

impl FromBCD for u128 {
    fn from_bcd(self) -> Self {
        let mut value = self;
        let mut result = 0;
        let mut digit_base = 1;
        while value != 0 {
            let lower = (value & 0xff) as u8;
            let lower = lower.from_bcd() as Self;
            result += lower*digit_base;
            value >>= 8;
            digit_base *= 100;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
