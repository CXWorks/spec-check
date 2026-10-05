pub open spec fn protocol_version__3_10_3_1_spec(status: i32, version: u32) -> bool {
    (status == 0 ==> version == 0x30000u32)
}
