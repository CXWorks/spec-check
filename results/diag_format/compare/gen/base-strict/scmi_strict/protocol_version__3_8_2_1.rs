pub open spec fn protocol_version__3_8_2_1_spec(result: int32, version: uint32, old_s: S, new_s: S) -> bool {
    (IsSuccessStatus(result) ==> version == 0x30001)
    && (IsSuccessStatus(result) ==> ProtocolVersionIs(version, 3, 1))
    && (old_s == new_s)
}