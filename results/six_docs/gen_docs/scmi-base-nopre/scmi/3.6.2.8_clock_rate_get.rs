pub open spec fn clock_rate_get_spec(result: int32, rate: [UInt32; 2], old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> !ClockExists(old_s, clock_id))
    && (result == SUCCESS ==> ClockExists(old_s, clock_id) && rate == old_s.clock_rate)
}