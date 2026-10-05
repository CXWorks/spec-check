pub open spec fn negotiate_protocol_version__3_8_2_2_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == NOT_SUPPORTED ==> <version not supported>)
    && (result == SUCCESS ==> <version supported>)
}