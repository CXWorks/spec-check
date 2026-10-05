pub open spec fn clock_rate_set_complete__3_6_3_1_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (ClockHasOtherUsers(old_s, clock_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (ClockAt(new_s, clock_id).rate == ((rate[1] as int) << 32) | (rate[0] as int)))
}