pub open spec fn protocol_attributes__3_3_2_3_spec(status: i32, attributes: u32, statistics_address_low: u32, statistics_address_high: u32, statistics_len: u32, old_s: S, new_s: S) -> bool {
    ((attributes & 0xFFFF_0000u32) == 0u32)
    && (new_s == old_s)
}
