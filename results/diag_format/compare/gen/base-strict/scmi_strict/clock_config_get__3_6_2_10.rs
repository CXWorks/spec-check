pub open spec fn clock_config_get__3_6_2_10_spec(result: Int32, attributes: UInt32, config: UInt32, extended_config_val: UInt32, old_s: S, new_s: S) -> bool {
    (!ClockExists(clock_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidConfigFlags(clock_id(old_s), flags(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && ResultEqual(result, SUCCESS) ==> (attributes == 0)
    && ResultEqual(result, SUCCESS) ==> (Bits(config, 31, 1) == 0)
    && ResultEqual(result, SUCCESS) ==> (Bits(config, 0, 0) == ClockEnableState(clock_id(old_s)))
    && ResultEqual(result, SUCCESS) ==> (Bits(flags(old_s), 7, 0) != 0 ==> extended_config_val == ExtendedConfigValue(clock_id(old_s), Bits(flags(old_s), 7, 0)))
    && true
}