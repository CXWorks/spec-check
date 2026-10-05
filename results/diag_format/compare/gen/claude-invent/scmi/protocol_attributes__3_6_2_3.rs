pub open spec fn protocol_attributes__3_6_2_3_spec(status: i32, attributes: u32, old_s: S, new_s: S) -> bool {
    (status == 0 ==> (
        ((attributes >> 24u32) & 0xFFu32) == 0u32
        && (((attributes >> 16u32) & 0xFFu32) as int) == ClockMaxPendingAsyncRateChanges(old_s)
        && ((attributes & 0xFFFFu32) as int) == ClockNumberOfClocks(old_s)
    ))
    && new_s == old_s
}
