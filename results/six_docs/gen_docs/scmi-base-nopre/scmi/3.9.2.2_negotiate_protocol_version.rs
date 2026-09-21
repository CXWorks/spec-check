pub open spec fn 3.9.2.2_negotiate_protocol_version_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (new_s.protocol_version == old_s.protocol_version || new_s.protocol_version == result as int))
    && (result != 0 ==> (result == -1))
}