use thiserror::Error;

use crate::{ComponentName, ContractVersion};

/// Projection between two versions of one logical type.
///
/// Forward and reverse projections use the same trait with the type
/// parameters swapped.
pub trait VersionProjection<Source, Target> {
    type Error;

    fn project(source: Source) -> Result<Target, Self::Error>;
}

/// Marker implemented by every type that participates in version
/// projection.
pub trait Projected: Sized {
    const CONTRACT_VERSION: ContractVersion;

    fn component() -> ComponentName;
}

/// Identity projection for unchanged types.
pub struct Identity;

impl<T: Projected> VersionProjection<T, T> for Identity {
    type Error = std::convert::Infallible;

    fn project(value: T) -> Result<T, Self::Error> {
        Ok(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ProjectionError {
    #[error("{source_type} cannot be represented by {target_type}")]
    NotRepresentable {
        source_type: String,
        target_type: String,
    },

    #[error("projection transform failed: {0}")]
    TransformFailed(String),

    #[error("projection direction is not implemented")]
    DirectionNotImplemented,
}
