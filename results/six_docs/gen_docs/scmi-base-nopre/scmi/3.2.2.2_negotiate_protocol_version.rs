pub open spec fn negotiate_protocol_version_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == NOT_SUPPORTED ==> !VersionEqual(new_s.protocol_version, old_s.protocol_version))
    && (result == SUCCESS ==> VersionEqual(new_s.protocol_version, old_s.protocol_version))
}