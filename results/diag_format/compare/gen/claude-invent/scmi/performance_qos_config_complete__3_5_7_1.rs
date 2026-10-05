pub open spec fn performance_qos_config_complete__3_5_7_1_spec(status: i32, domain_id: u32, capability: u32, flags: u32, qos_value: u32) -> bool {
    ((flags & 0xFFFF_FFE0u32) == 0u32)
    && ((flags & 0x0000_0003u32) == 0u32)
    && ((((flags >> 4u32) & 1u32) == 1u32) ==> (domain_id == 0xFFFF_FFFFu32))
}
