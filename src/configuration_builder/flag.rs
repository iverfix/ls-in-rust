use std::convert::TryFrom;

pub enum Flag {
    G,
    CapG,
    O,
    L,
    A,
}

#[derive(Debug)]
pub enum FlagParseError {
    UnknownFlag(String),
}

impl Flag {
    pub fn try_from_long(value: &str) -> Result<Flag, FlagParseError> {
        match value {
            "no-group" => Ok(Flag::G),
            "all" => Ok(Flag::A),
            _ => Err(FlagParseError::UnknownFlag(value.to_string())),
        }
    }
}

impl TryFrom<&str> for Flag {
    type Error = FlagParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "g" => Ok(Flag::G),
            "G" => Ok(Flag::CapG),
            "o" => Ok(Flag::O),
            "l" => Ok(Flag::L),
            "a" => Ok(Flag::A),
            _ => Err(FlagParseError::UnknownFlag(value.to_string())),
        }
    }
}
