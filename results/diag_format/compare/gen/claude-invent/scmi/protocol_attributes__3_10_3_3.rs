pub open spec fn protocol_attributes__3_10_3_3_spec(status: i32, attributes: u32, old_s: S, new_s: S) -> bool {
    (status == 0 ==> (
        ((attributes >> 16u32) & 0xFFFFu32) == 0u32
        && ((attributes & 0xFFFFu32) as int) == (NumPowerCappingDomains(old_s) as int)
    ))
    && new_s == old_s
}
