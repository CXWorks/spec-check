pub open spec fn protocol_version__3_6_2_1_spec(status: i32, version: UInt32, old_s: S, new_s: S) -> bool {
    (status == 0 ==> version == 0x30000)
    && new_s == old_s
}
