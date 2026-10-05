pub open spec fn protocol_version__3_11_2_1_spec(status: i32, version: u32, old_s: S, new_s: S) -> bool {
    (status == 0 ==> version == 0x10000u32)
    && (new_s == old_s)
}
