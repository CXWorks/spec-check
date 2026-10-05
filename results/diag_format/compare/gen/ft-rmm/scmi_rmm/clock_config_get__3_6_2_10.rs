pub open spec fn clock_config_get__3_6_2_10_spec(clock_id: UInt32, flags: UInt32, result: Result<Int32, (Int32, UInt32, UInt32, UInt32)>, old_s: S, new_s: S) -> bool {
  (!ClockExists(old_s, clock_id) ==> ResultEqual(result.0, NOT_FOUND))
  && (!IsSupportedConfig(old_s, flags) ==> ResultEqual(result.0, INVALID_PARAMETERS))
  && (result == SUCCESS ==> result.1 == 0)
  && (result == SUCCESS ==> result.2[0] == (Clock(new_s, clock_id).enabled ? 1 : 0))
  && ((flags[7:0] != 0) ==> result.3 == ExtendedConfigValue(new_s, clock_id, flags[7:0]))
  && ((!(ClockExists(old_s, clock_id)) &&
       IsSupportedConfig(old_s, flags))
    ==> ResultEqual(result.0, SUCCESS))
}