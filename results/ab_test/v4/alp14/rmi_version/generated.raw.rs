```rust
pub open spec fn RMI_VERSION_spec(s: S, req: RmiInterfaceVersion, result: Result<(), RmiStatusCode>, lower: RmiInterfaceVersion, higher: RmiInterfaceVersion) -> bool {
    ((!RmiVersionIsSupported(s, req) && RmiVersionLowerIsSupported(s, req))
        ==> (ResultEqual(result, RMI_ERROR_INPUT)
            && VersionEqual(lower, RmiVersionHighestBelow(s, req))
            && VersionEqual(higher, RmiVersionHighest(s))))
    && ((!RmiVersionIsSupported(s, req) && !RmiVersionLowerIsSupported(s, req) && RmiVersionHigherIsSupported(s, req))
        ==> (ResultEqual(result, RMI_ERROR_INPUT)
            && VersionEqual(lower, higher)
            && VersionEqual(higher, RmiVersionHighest(s))))
    && (RmiVersionIsSupported(s, req)
        ==> (result.is_Ok()
            && VersionEqual(lower, req)
            && VersionEqual(higher, RmiVersionHighest(s))))
}
```