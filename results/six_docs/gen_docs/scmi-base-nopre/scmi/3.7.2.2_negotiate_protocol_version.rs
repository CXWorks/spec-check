pub open spec fn negotiate_protocol_version_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == NOT_SUPPORTED ==> !VersionSupported(old_s, version))
    && (result == SUCCESS ==> VersionSupported(old_s, version))
}