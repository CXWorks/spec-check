pub open spec fn protocol_version__3_5_6_1_spec(status: i32, version: u32) -> bool {
    (status == 0 ==> version == 0x40001u32)
}
