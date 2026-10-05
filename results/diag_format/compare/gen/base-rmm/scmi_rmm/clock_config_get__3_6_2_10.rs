pub open spec fn clock_config_get__3_6_2_10_spec(result: Int32, attributes: UInt32, config: UInt32, extended_config_val: UInt32, old_s: S, new_s: S) -> bool {
    (!ClockExists(clock_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!IsSupportedConfig(flags(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && ResultEqual(result, SUCCESS) ==> (config[0] == (Clock(old_s, clock_id(old_s)).enabled ? 1 : 0))
    && ((flags(old_s)[7:0] != 0) ==> (ResultEqual(result, SUCCESS) ==> extended_config_val == ExtendedConfigValue(clock_id(old_s), flags(old_s)[7:0])))
    && (attributes == 0)
    && (config[31:1] == 0)
    && (new_s == old_s)
}