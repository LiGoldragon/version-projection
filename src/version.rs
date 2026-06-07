use nota_next::{Block, NotaDecode, NotaDecodeError, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use thiserror::Error;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentName(String);

impl ComponentName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl NotaDecode for ComponentName {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        String::from_nota_block(block).map(Self::new)
    }
}

impl NotaEncode for ComponentName {
    fn to_nota(&self) -> String {
        self.0.clone()
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct ContractVersion([u8; 32]);

impl ContractVersion {
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub const fn zero() -> Self {
        Self([0; 32])
    }

    pub const fn bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn try_from_bytes(bytes: Vec<u8>) -> Result<Self, ContractVersionError> {
        let length = bytes.len();
        let bytes: [u8; 32] = bytes
            .try_into()
            .map_err(|_| ContractVersionError::InvalidLength { length })?;
        Ok(Self(bytes))
    }

    pub fn try_from_nota_literal(literal: &str) -> Result<Self, ContractVersionError> {
        let hex =
            literal
                .strip_prefix('#')
                .ok_or_else(|| ContractVersionError::InvalidLiteral {
                    value: literal.to_owned(),
                })?;
        let length = hex.len() / 2;
        if hex.len() != 64 {
            return Err(ContractVersionError::InvalidLength { length });
        }

        let mut bytes = [0_u8; 32];
        for (index, pair) in hex.as_bytes().chunks_exact(2).enumerate() {
            let high = Self::hex_digit(pair[0], literal)?;
            let low = Self::hex_digit(pair[1], literal)?;
            bytes[index] = (high << 4) | low;
        }
        Ok(Self(bytes))
    }

    pub fn to_nota_literal(&self) -> String {
        let mut literal = String::with_capacity(65);
        literal.push('#');
        for byte in self.0 {
            literal.push_str(&format!("{byte:02x}"));
        }
        literal
    }

    fn hex_digit(byte: u8, literal: &str) -> Result<u8, ContractVersionError> {
        match byte {
            b'0'..=b'9' => Ok(byte - b'0'),
            b'a'..=b'f' => Ok(byte - b'a' + 10),
            b'A'..=b'F' => Ok(byte - b'A' + 10),
            _ => Err(ContractVersionError::InvalidLiteral {
                value: literal.to_owned(),
            }),
        }
    }
}

impl NotaDecode for ContractVersion {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let literal = block
            .demote_to_string()
            .ok_or(NotaDecodeError::ExpectedAtom {
                type_name: "ContractVersion",
            })?;
        Self::try_from_nota_literal(literal)
            .map_err(|error| NotaDecodeError::Parse(error.to_string()))
    }
}

impl NotaEncode for ContractVersion {
    fn to_nota(&self) -> String {
        self.to_nota_literal()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ContractVersionError {
    #[error("contract version hash must be 32 bytes, got {length}")]
    InvalidLength { length: usize },
    #[error("contract version must be a #hex byte literal: {value}")]
    InvalidLiteral { value: String },
}
