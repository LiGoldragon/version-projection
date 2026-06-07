use nota_next::{NotaDecode, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::ComponentName;

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum OperationKind {
    AppendWrite,
    MutateWrite,
    RetractWrite,
    Match,
    Subscribe,
    Tap,
}

impl OperationKind {
    pub const fn as_record_head(self) -> &'static str {
        match self {
            Self::AppendWrite => "AppendWrite",
            Self::MutateWrite => "MutateWrite",
            Self::RetractWrite => "RetractWrite",
            Self::Match => "Match",
            Self::Subscribe => "Subscribe",
            Self::Tap => "Tap",
        }
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum WritePolicy {
    Mirror,
    DivergenceRecord,
    Reject,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum ReadPolicy {
    ActiveProjectsResponse,
    DualQueryMerge,
    ActiveOnly,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum SubscribePolicy {
    ResumeAgainstNext,
    #[default]
    TerminateAtHandover,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct PerOperationPolicy {
    pub write_policy: WritePolicy,
    pub read_policy: ReadPolicy,
    pub subscribe_policy: SubscribePolicy,
}

impl PerOperationPolicy {
    pub const fn new(
        write_policy: WritePolicy,
        read_policy: ReadPolicy,
        subscribe_policy: SubscribePolicy,
    ) -> Self {
        Self {
            write_policy,
            read_policy,
            subscribe_policy,
        }
    }

    pub const fn mirror_append() -> Self {
        Self::new(
            WritePolicy::Mirror,
            ReadPolicy::ActiveOnly,
            SubscribePolicy::TerminateAtHandover,
        )
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct OperationPolicy {
    pub kind: OperationKind,
    pub policy: PerOperationPolicy,
}

impl OperationPolicy {
    pub const fn new(kind: OperationKind, policy: PerOperationPolicy) -> Self {
        Self { kind, policy }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ComponentPolicy {
    pub component: ComponentName,
    pub operations: Vec<OperationPolicy>,
}

impl ComponentPolicy {
    pub fn new(component: ComponentName, operations: Vec<OperationPolicy>) -> Self {
        Self {
            component,
            operations,
        }
    }
}
