pub open spec fn protocol_version__3_8_2_1_spec(result: Int32, version: UInt32, old_s: S, new_s: S) -> bool {
    (true ==> result == 0)
    && (true ==> version == 0x30001)
    && (true ==> old_s == new_s)
}