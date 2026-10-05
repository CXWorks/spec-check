pub open spec fn negotiate_protocol_version__3_5_6_2_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (new_s.protocol_version == old_s.version))
    && (result != 0 ==> (new_s.protocol_version != old_s.version))
}