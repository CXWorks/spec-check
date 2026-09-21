pub open spec fn clock_rate_set_complete_spec(result: int, clock_id: UInt32, rate: [UInt32; 2], old_s: S, new_s: S) -> bool {
    (result == 0x5)
    && (clock_id == old_s.clock_id)
    && (rate[0] as int == old_s.rate[0] as int)
    && (rate[1] as int == old_s.rate[1] as int)
}