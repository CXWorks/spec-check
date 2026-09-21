pub open spec fn base_discover_implementation_version_spec(result: i32, implementation_version: u32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> implementation_version == old_s.implementation_version)
    && (result != 0 ==> true)
}