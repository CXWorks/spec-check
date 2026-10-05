pub open spec fn clock_rate_get__3_6_2_8_spec(clock_id: UInt32, status: Int32, rate: [UInt32; 2], old_s: S, new_s: S) -> bool {
  (!ClockExists(old_s, clock_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> (rate[1] << 32 | rate[0]) == ClockCurrentRate(old_s, clock_id))
  && (ResultEqual(status, SUCCESS) && ClockIsEnabled(old_s, clock_id) ==> (rate[1] << 32 | rate[0]) == ClockCurrentRate(old_s, clock_id))
  && (ResultEqual(status, SUCCESS) && !ClockIsEnabled(old_s, clock_id) ==> (rate[1] << 32 | rate[0]) == ClockRateOnReenable(old_s, clock_id))
  && ((ClockExists(old_s, clock_id))
    ==> ResultEqual(status, SUCCESS))
}