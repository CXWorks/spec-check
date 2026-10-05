pub open spec fn clock_rate_set_complete__3_6_3_1_spec(clock_id: UInt32, rate_0: UInt32, rate_1: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (ClockHasOtherUsers(old_s, clock_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> ClockAt(new_s, clock_id).rate == (rate_1 << 32) | rate_0)
  && (ResultEqual(status, SUCCESS) ==> status == SUCCESS)
  && ((!ClockHasOtherUsers(old_s, clock_id))
    ==> ResultEqual(status, SUCCESS))
}