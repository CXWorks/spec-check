pub open spec fn protocol_version__3_9_2_1_spec(status: i32, version: u32, old_s: S, new_s: S) -> bool {
    (status == 0 ==> version == 0x20001u32)
    && new_s == old_s
}
