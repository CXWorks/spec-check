pub open spec fn negotiate_protocol_version__3_10_3_2_spec(result: int32, old_s: S, new_s: S) -> bool {
    (version_not_supported(old_s, version) ==> ResultEqual(result, NOT_SUPPORTED))
    && (version_supported(old_s, version) ==> result == SUCCESS)
}

fn version_not_supported(old_s: S, version: uint32) -> bool {
    !version_is_supported(old_s, version)
}

fn version_supported(old_s: S, version: uint32) -> bool {
    version <= old_s.protocol_version_max
}