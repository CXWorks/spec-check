pub open spec fn clock_rate_change_requested_notify__3_6_2_13_spec(clock_id: uint32, notify_enable: uint32, status: int32, old_s: S, new_s: S) -> bool {
  (!IsValidClockId(old_s, clock_id) ==> ResultEqual(status, NOT_FOUND))
  && (result == SUCCESS ==> ResultEqual(status, SUCCESS))
  && (result == SUCCESS ==> RateChangeRequestedNotifyEnabled(new_s, calling_agent, clock_id) == notify_enable[0])
  && ((!(IsValidClockId(old_s, clock_id)))
    ==> ResultEqual(status, SUCCESS))
}