pub open spec fn clock_rate_set_complete__3_6_3_1_spec(result: Int32, clock_id: UInt32, rate_lo: UInt32, rate_hi: UInt32, old_s: S, new_s: S) -> bool {
    (ClockHasOtherUsers(clock_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (ResultEqual(result, SUCCESS) && (ClockRate(clock_id) == rate_lo + rate_hi * 4294967296)))
}