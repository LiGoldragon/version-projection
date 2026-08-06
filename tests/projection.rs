use std::process::Command;

#[cfg(feature = "dotos-text")]
use dotos::{DotosEncode, DotosSource};
use version_projection::{
    ComponentName, ContractVersion, DecodeError, Identity, OperationKind, PerOperationPolicy,
    Projected, RecordKind, RuntimeMigrationLookup, RuntimeMigrationLookupEntry, SubscribePolicy,
    VersionProjection,
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
#[cfg(feature = "dotos-text")]
fn contract_version_projects_to_dotos_byte_literal() {
    let version = ContractVersion::new([1; 32]);
    let text = version.to_dotos();

    assert_eq!(
        text,
        "#0101010101010101010101010101010101010101010101010101010101010101"
    );
    let decoded = DotosSource::new(&text)
        .parse::<ContractVersion>()
        .expect("decode");
    assert_eq!(decoded, version);
}

#[test]
#[cfg(feature = "dotos-text")]
fn component_and_record_names_project_to_wire_tokens() {
    let component = ComponentName::new("persona-spirit");
    let kind = RecordKind::new("Entry");

    assert_eq!(component.to_dotos(), "persona-spirit");
    assert_eq!(kind.to_dotos(), "Entry");
    assert_eq!(
        DotosSource::new("persona-spirit")
            .parse::<ComponentName>()
            .expect("decode component")
            .as_str(),
        component.as_str()
    );
    assert_eq!(
        DotosSource::new("Entry")
            .parse::<RecordKind>()
            .expect("decode record kind")
            .as_str(),
        kind.as_str()
    );
}

#[test]
fn default_dependency_tree_does_not_pull_dotos() {
    let output = Command::new(env!("CARGO"))
        .args(["tree", "--edges", "normal", "--no-default-features"])
        .output()
        .expect("run cargo tree");

    assert!(output.status.success(), "status: {:?}", output.status);
    let tree = String::from_utf8(output.stdout).expect("cargo tree output");
    assert!(
        !tree.lines().any(|line| line.contains("dotos v")),
        "default dependency tree unexpectedly contains dotos:\n{tree}"
    );
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

#[test]
fn runtime_migration_lookup_finds_decoder_by_component_and_contract_version() {
    fn decode_record(bytes: &[u8], kind: &RecordKind) -> Result<String, DecodeError> {
        let text =
            std::str::from_utf8(bytes).map_err(|error| DecodeError::Failed(error.to_string()))?;
        Ok(format!("{}:{text}", kind.as_str()))
    }

    let component = ComponentName::new("test-component");
    let contract_version = ContractVersion::new([9; 32]);
    let entry =
        RuntimeMigrationLookupEntry::new(component.clone(), contract_version, decode_record);
    let lookup = RuntimeMigrationLookup::new(vec![entry]);

    let found = lookup
        .find(&component, contract_version)
        .expect("matching entry");

    assert_eq!(found.component().as_str(), "test-component");
    assert_eq!(found.contract_version(), contract_version);
    assert_eq!(
        found
            .decode(b"hello", &RecordKind::new("Entry"))
            .expect("decode"),
        "Entry:hello"
    );
    assert!(
        lookup
            .find(&ComponentName::new("other-component"), contract_version)
            .is_none()
    );
}
