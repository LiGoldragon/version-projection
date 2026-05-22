use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use thiserror::Error;

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct ComponentName(String);

impl ComponentName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
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
}

impl NotaEncode for ContractVersion {
    fn encode(&self, encoder: &mut Encoder) -> nota_codec::Result<()> {
        encoder.write_bytes(&self.0)
    }
}

impl NotaDecode for ContractVersion {
    fn decode(decoder: &mut Decoder<'_>) -> nota_codec::Result<Self> {
        let bytes = decoder.read_bytes()?;
        Self::try_from_bytes(bytes).map_err(|error| nota_codec::Error::Validation {
            type_name: "ContractVersion",
            message: error.to_string(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ContractVersionError {
    #[error("contract version hash must be 32 bytes, got {length}")]
    InvalidLength { length: usize },
}
