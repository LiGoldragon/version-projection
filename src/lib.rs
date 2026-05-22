//! Version projection primitives.
//!
//! The crate is intentionally library-only. Runtime crates use these
//! types to project values across adjacent component versions and to
//! describe the policy for what happens when projection succeeds or
//! fails.

pub mod index;
pub mod policy;
pub mod projection;
pub mod version;

pub use index::{DecodeError, MigrationIndex, MigrationIndexEntry, RecordKind};
pub use policy::{
    ComponentPolicy, OperationKind, OperationPolicy, PerOperationPolicy, ReadPolicy,
    SubscribePolicy, WritePolicy,
};
pub use projection::{Identity, Projected, ProjectionError, VersionProjection};
pub use version::{ComponentName, ContractVersion, ContractVersionError};
