pub open spec fn clock_config_get__3_6_2_10_spec(clock_id: UInt32, flags: UInt32, status: Int32, attributes: UInt32, config: UInt32, extended_config_val: UInt32, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!ClockExists(old_s, clock_id) ==> ResultEqual(result, NOT_FOUND))
  && (!IsValidConfigFlags(old_s, clock_id, flags) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result.is_Ok() ==> ResultEqual(result, SUCCESS))
  && (result.is_Ok() ==> attributes == 0)
  && (result.is_Ok() ==> (config & 0x80000000) == 0)
  && (result.is_Ok() ==> (config & 1) == ClockEnableState(old_s, clock_id))
  && (result.is_Ok() && (flags & 0xFF) != 0 ==> extended_config_val == ExtendedConfigValue(old_s, clock_id, (flags & 0xFF) as int))
  && ((!(ClockExists(old_s, clock_id)) &&
       IsValidConfigFlags(old_s, clock_id, flags))
    ==> ResultEqual(result, SUCCESS))
  && (result.is_Err()
    ==> attributes == 0)
  && (result.is_Err()
    ==> (config & 0x80000000) == 0)
  && (result.is_Err()
    ==> (config & 1) == ClockEnableState(old_s, clock_id))
  && (!(result.is_Ok() && (flags & 0xFF) != 0) ==> extended_config_val == ExtendedConfigValue(old_s, clock_id, (flags & 0xFF) as int))
}