pub open spec fn clock_rate_notify__3_6_2_12_spec(clock_id: UInt32, notify_enable: UInt32, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!IsValidClockDevice(old_s, clock_id) ==> ResultEqual(result, NOT_FOUND))
  && (!IsValidNotifyEnable(old_s, notify_enable) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result.is_Ok() ==> ResultEqual(result, SUCCESS))
  && (result.is_Ok() && Bits(notify_enable, 0, 0) == 1 ==> ClockRateNotifyEnabled(new_s, CallingAgent(), clock_id))
  && (result.is_Ok() && Bits(notify_enable, 0, 0) == 0 ==> !ClockRateNotifyEnabled(new_s, CallingAgent(), clock_id))
  && ((IsValidClockDevice(old_s, clock_id) &&
       IsValidNotifyEnable(old_s, notify_enable))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> !ClockRateNotifyEnabled(new_s, CallingAgent(), clock_id))
}