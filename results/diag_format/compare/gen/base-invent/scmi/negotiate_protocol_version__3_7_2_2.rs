pub open spec fn negotiate_protocol_version__3_7_2_2_spec(result: int32, old_s: S, new_s: S) -> bool {
    (version_not_supported(old_s, result) ==> result == NOT_SUPPORTED)
    && (version_supported(old_s, result) ==> result == SUCCESS)
}

fn version_not_supported(old_s: S, result: int32) -> bool {
    result != SUCCESS
}

fn version_supported(old_s: S, result: int32) -> bool {
    result == SUCCESS
}