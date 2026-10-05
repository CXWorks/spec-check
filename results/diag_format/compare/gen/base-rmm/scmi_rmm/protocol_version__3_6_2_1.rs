pub open spec fn protocol_version__3_6_2_1_spec(result: Int32, version: UInt32, old_s: S, new_s: S) -> bool {
    (IsSuccessStatus(result) ==> version == 0x30000)
    && (old_s == new_s)
}