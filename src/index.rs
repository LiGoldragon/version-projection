#[cfg(feature = "dotos-text")]
use dotos::{Block, DotosDecode, DotosDecodeError, DotosEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use thiserror::Error;

use crate::{ComponentName, ContractVersion};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordKind(String);

impl RecordKind {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(feature = "dotos-text")]
impl DotosDecode for RecordKind {
    fn from_dotos_block(block: &Block) -> Result<Self, DotosDecodeError> {
        String::from_dotos_block(block).map(Self::new)
    }
}

#[cfg(feature = "dotos-text")]
impl DotosEncode for RecordKind {
    fn to_dotos(&self) -> String {
        self.0.clone()
    }
}

pub type DecodeFunction = fn(&[u8], &RecordKind) -> Result<String, DecodeError>;

#[derive(Clone)]
pub struct RuntimeMigrationLookupEntry {
    component: ComponentName,
    contract_version: ContractVersion,
    decode: DecodeFunction,
}

impl RuntimeMigrationLookupEntry {
    pub fn new(
        component: ComponentName,
        contract_version: ContractVersion,
        decode: DecodeFunction,
    ) -> Self {
        Self {
            component,
            contract_version,
            decode,
        }
    }

    pub fn component(&self) -> &ComponentName {
        &self.component
    }

    pub const fn contract_version(&self) -> ContractVersion {
        self.contract_version
    }

    pub fn decode(&self, bytes: &[u8], kind: &RecordKind) -> Result<String, DecodeError> {
        (self.decode)(bytes, kind)
    }
}

#[derive(Clone, Default)]
pub struct RuntimeMigrationLookup {
    entries: Vec<RuntimeMigrationLookupEntry>,
}

impl RuntimeMigrationLookup {
    pub fn new(entries: Vec<RuntimeMigrationLookupEntry>) -> Self {
        Self { entries }
    }

    pub fn entries(&self) -> &[RuntimeMigrationLookupEntry] {
        &self.entries
    }

    pub fn find(
        &self,
        component: &ComponentName,
        contract_version: ContractVersion,
    ) -> Option<&RuntimeMigrationLookupEntry> {
        self.entries.iter().find(|entry| {
            entry.component.as_str() == component.as_str()
                && entry.contract_version == contract_version
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DecodeError {
    #[error("no decoder for record kind {0}")]
    UnknownRecordKind(String),

    #[error("decode failed: {0}")]
    Failed(String),
}
