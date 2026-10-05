pub open spec fn clock_rate_notify__3_6_2_12_spec(clock_id: UInt32, notify_enable: UInt32, result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  ((notify_enable & 1) == 0 ==> result == RSI_SUCCESS)
  && (result == RSI_NOT_FOUND ==> (ClockAt(new_s, clock_id as int).notify_enable == ClockAt(old_s, clock_id as int).notify_enable))
  && ((!( (notify_enable & 1) == 0 ))
    ==> result == RSI_SUCCESS)
}