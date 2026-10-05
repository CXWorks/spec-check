pub open spec fn protocol_version__3_5_6_1_spec(result: int32, version: uint32, old_s: S, new_s: S) -> bool {
    (StatusIndicatesSuccess(result) ==> version == 0x40001)
    && (version == 0x40001 ==> StatusIndicatesSuccess(result))
    && (old_s == new_s)
}