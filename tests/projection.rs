use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use version_projection::{
    ComponentName, ContractVersion, Identity, OperationKind, PerOperationPolicy, Projected,
    SubscribePolicy, VersionProjection,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Entry {
    text: String,
}

impl Entry {
    fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Projected for Entry {
    const CONTRACT_VERSION: ContractVersion = ContractVersion::new([7; 32]);

    fn component() -> ComponentName {
        ComponentName::new("test-component")
    }
}

#[test]
fn identity_projection_returns_unchanged_projected_type() {
    let entry = Entry::new("same");

    let projected = <Identity as VersionProjection<Entry, Entry>>::project(entry.clone())
        .expect("identity projection cannot fail");

    assert_eq!(projected, entry);
}

#[test]
fn contract_version_projects_to_nota_byte_literal() {
    let version = ContractVersion::new([1; 32]);
    let mut encoder = Encoder::new();

    version.encode(&mut encoder).expect("encode");
    let text = encoder.into_string();

    assert_eq!(
        text,
        "#0101010101010101010101010101010101010101010101010101010101010101"
    );
    let mut decoder = Decoder::new(&text);
    let decoded = ContractVersion::decode(&mut decoder).expect("decode");
    assert_eq!(decoded, version);
}

#[test]
fn subscription_policy_defaults_to_terminate_at_handover() {
    assert_eq!(
        SubscribePolicy::default(),
        SubscribePolicy::TerminateAtHandover
    );
}

#[test]
fn policy_records_keep_operation_kind_separate_from_projection_trait() {
    let policy = PerOperationPolicy::mirror_append();

    assert_eq!(OperationKind::AppendWrite.as_record_head(), "AppendWrite");
    assert_eq!(
        policy.subscribe_policy,
        SubscribePolicy::TerminateAtHandover
    );
}
