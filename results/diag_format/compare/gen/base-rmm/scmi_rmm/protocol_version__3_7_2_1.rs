pub open spec fn protocol_version__3_7_2_1_spec(result: int32, version: uint32, old_s: S, new_s: S) -> bool {
    (true ==> result == 0)
    && (true ==> version == 0x30001)
    && (true ==> old_s == new_s)
}