pub open spec fn protocol_version__3_2_2_1_spec(status: i32, version: UInt32, old_s: S, new_s: S) -> bool {
    (version == 0x20001u32)
    && (new_s == old_s)
}
