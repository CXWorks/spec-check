pub open spec fn clock_rate_set_complete__3_6_3_1_spec(clock_id: UInt32, rate: [UInt32; 2], status: Int32, old_s: S, new_s: S) -> bool {
  (ClockHasOtherUsers(old_s, clock_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> ClockRate(new_s, clock_id) == rate[0] + rate[1] * 4294967296)
  && ((!(ClockHasOtherUsers(old_s, clock_id)))
    ==> ResultEqual(status, SUCCESS))
}