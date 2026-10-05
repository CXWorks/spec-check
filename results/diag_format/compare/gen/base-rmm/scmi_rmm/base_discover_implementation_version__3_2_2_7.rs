pub open spec fn base_discover_implementation_version_spec(result: int32, implementation_version: uint32, old_s: S, new_s: S) -> bool {
    (true ==> result == 0)
    && (true ==> implementation_version == VendorImplementationVersion())
    && (true ==> implementation_version > VendorImplementationVersionOfAnyOlderImplementation(old_s))
    && (old_s == new_s)
}