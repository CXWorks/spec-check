pub open spec fn protocol_attributes__3_9_2_3_spec(status: i32, attributes: u32, old_s: S, new_s: S) -> bool {
    (status == 0 ==> (
        (attributes >> 16u32) == 0u32
        && ((attributes & 0xFFFFu32) as int) == NumVoltageDomains(old_s)
    ))
    && new_s == old_s
}
