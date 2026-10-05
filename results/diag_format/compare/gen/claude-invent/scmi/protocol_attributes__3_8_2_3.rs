pub open spec fn protocol_attributes__3_8_2_3_spec(status: i32, attributes: UInt32, old_s: S, new_s: S) -> bool {
    (status == 0 ==> (
        ((attributes >> 16u32) & 0xFFFFu32) == 0u32
        && ((attributes & 0xFFFFu32) as int) == NumResetDomains(old_s)
    ))
    && new_s == old_s
}
