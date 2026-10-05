pub open spec fn clock_rate_get__3_6_2_8_spec(clock_id: UInt32, status: Int32, rate_low: UInt32, rate_high: UInt32, old_s: S, new_s: S) -> bool {
  (!ClockExists(old_s, clock_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> !ClockIsDisabled(old_s, clock_id) ==> rate_low + rate_high * 4294967296 == ClockCurrentRate(old_s, clock_id))
  && (ResultEqual(status, SUCCESS) ==> ClockIsDisabled(old_s, clock_id) ==> rate_low + rate_high * 4294967296 == ClockRateOnReenable(old_s, clock_id))
  && ((ClockExists(old_s, clock_id))
    ==> ResultEqual(status, SUCCESS))
}