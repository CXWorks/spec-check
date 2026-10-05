pub open spec fn protocol_version__3_6_2_1_spec(result: int32, version: uint32, old_s: S, new_s: S) -> bool {
    (true ==> IsSuccessStatus(result))
    && (true ==> version == 0x30000)
}