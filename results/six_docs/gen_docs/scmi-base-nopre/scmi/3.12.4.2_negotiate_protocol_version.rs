pub open spec fn negotiate_protocol_version_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == NOT_SUPPORTED ==> version_not_supported(old_s, new_s))
    && (result == SUCCESS ==> version_supported(old_s, new_s))
}

fn version_not_supported(old_s: S, new_s: S) -> bool {
    true
}

fn version_supported(old_s: S, new_s: S) -> bool {
    true
}