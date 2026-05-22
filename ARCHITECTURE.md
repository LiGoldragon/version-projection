# version-projection Architecture

## Purpose

`version-projection` is the shared library for the version-handover
foundation described in `reports/designer/285-versionprojection-trait-and-handover-protocol-specification.md`.

It owns:

- `VersionProjection<Source, Target>`, the bidirectional projection
  trait. Forward and reverse projection are the same relation with the
  type parameters swapped.
- `Projected`, the marker every projected signal or storage type
  implements to name its component and contract version.
- payloadless compatibility policy records used by runtime crates to
  decide whether an operation mirrors to `main`, records divergence, or
  rejects.
- compile-time migration-index records for locating historical
  decoders.

It does not own:

- daemon code;
- signal-frame transport;
- component runtime policy enforcement;
- persona-spirit-specific migration logic;
- schema hashing generation.

## Version Identity

`ContractVersion` stores the schema-version hash as 32 bytes. The NOTA
projection uses the byte-literal form so a schema hash remains a binary
identity, not a stringly version label.

## Projection Contract

Projection is a type relation:

```rust
VersionProjection<Source, Target>::project(source) -> Result<Target, Error>
```

The trait only says whether a value can be represented in the target
type. Runtime code decides what to do with that answer through
`PerOperationPolicy`:

- `Mirror`: `next` performs the operation and projects it back to
  `main`.
- `DivergenceRecord`: `next` performs the operation and records that
  `main` cannot represent it.
- `Reject`: `next` refuses the operation.

The identity case is covered by `Identity`, so unchanged types do not
need handwritten projection code.
