pub open spec fn clock_rate_change_requested_notify_3_6_2_13_spec(clock_id: UInt32, notify_enable: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidClockId(old_s, clock_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) && Bits(notify_enable, 0, 0) == 1 ==> RateChangeRequestedNotifyEnabled(new_s, CallingAgent(), clock_id))
  && (ResultEqual(status, SUCCESS) && Bits(notify_enable, 0, 0) == 0 ==> !RateChangeRequestedNotifyEnabled(new_s, CallingAgent(), clock_id))
  && ((IsValidClockId(old_s, clock_id))
    ==> ResultEqual(status, SUCCESS))
}