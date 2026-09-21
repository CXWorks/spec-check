pub open spec fn 3.6.2.2_negotiate_protocol_version_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> new_s.protocol_version == old_s.protocol_version)
    && (result == 0 ==> old_s.protocol_version == new_s.protocol_version)
    && (result != 0 ==> new_s.protocol_version == old_s.protocol_version)
    && (result != 0 ==> old_s.protocol_version == new_s.protocol_version)
}