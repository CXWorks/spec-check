pub open spec fn clock_rate_notify__3_6_2_12_spec(clock_id: UInt32, notify_enable: [UInt32; 1], result: Result<Int32, RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!IsValidClockDevice(old_s, clock_id) ==> ResultEqual(result, NOT_FOUND))
  && (!IsValidNotifyEnable(old_s, notify_enable[0]) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result == RSI_SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == RSI_SUCCESS ==> ClockRateNotifyEnabled(new_s, calling_agent, clock_id) == (notify_enable[0] == 1))
  && ((IsValidClockDevice(old_s, clock_id) &&
       IsValidNotifyEnable(old_s, notify_enable[0]))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> ClockRateNotifyEnabled(new_s, calling_agent, clock_id) == ClockRateNotifyEnabled(old_s, calling_agent, clock_id))
}