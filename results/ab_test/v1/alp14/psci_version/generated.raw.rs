```rust
pub open spec fn PSCI_VERSION_spec(s: S) -> bool {
    VersionEqual(PSCI_VERSION(), PsciVersion())
}
```

Or, if you need the full function signature with state parameters and return value:

```rust
pub open spec fn PSCI_VERSION_spec(s: S) -> PsciInterfaceVersion {
    PsciVersion()
}
```

Or, if modeling it as a specification with pre and post conditions:

```rust
pub open spec fn PSCI_VERSION_spec(s: S, result: PsciInterfaceVersion) -> bool {
    VersionEqual(result, PsciVersion())
}
```