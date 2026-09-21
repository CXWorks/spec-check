pub open spec fn 3.5.6.2_negotiate_protocol_version_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (new_s.protocol_version == old_s.version))
    && (result != 0 ==> (new_s.protocol_version != old_s.version))
}