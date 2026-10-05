pub open spec fn protocol_attributes__3_5_6_3_spec(status: i32, attributes: u32, statistics_address_low: u32, statistics_address_high: u32, statistics_len: u32, old_s: S, new_s: S) -> bool {
    (status == 0i32 ==> (
        ((attributes >> 18u32) == 0u32)
        && (((attributes >> 16u32) & 3u32) != 3u32)
        && (statistics_len != 0u32 ==> (statistics_address_low % 8u32) == 0u32)
    ))
    && (new_s == old_s)
}
