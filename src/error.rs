#[derive(Debug)]
pub struct BCDConversionError {
    description: String
}

impl BCDConversionError {
    pub fn new(description: impl Into<String>) -> Self {
        Self { description: description.into() }
    }

    pub fn new_boxed(description: String) -> Box<Self> {
        Box::new(Self::new(description))
    }

    pub fn new_with_template_description<T>(t: &str, v: T, max: T) -> Self 
        where T: core::fmt::Display
    {
        Self::new(format!("Error on {} to bcd, passed in value ({}) exceeds maximum of {}", t, v,max))
    }

    pub fn description<'self_lt>(&'self_lt self) -> &'self_lt str {
        &self.description
    }
}

impl core::fmt::Display for BCDConversionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for BCDConversionError {}

