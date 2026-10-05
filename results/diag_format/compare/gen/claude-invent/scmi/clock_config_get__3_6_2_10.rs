pub open spec fn clock_config_get__3_6_2_10_spec(clock_id: u32, flags: u32, status: i32, attributes: u32, config: u32, extended_config_val: u32, old_s: S, new_s: S) -> bool {
    (!ClockExists(old_s, clock_id) ==> status == NOT_FOUND)
    && ((ClockExists(old_s, clock_id)
            && ((flags >> 8u32) != 0u32
                || ((flags & 0xFFu32) != 0u32
                    && !IsExtendedConfigTypeSupported(old_s, clock_id, flags & 0xFFu32))))
        ==> status == INVALID_PARAMETERS)
    && ((ClockExists(old_s, clock_id)
            && (flags >> 8u32) == 0u32
            && ((flags & 0xFFu32) == 0u32
                || IsExtendedConfigTypeSupported(old_s, clock_id, flags & 0xFFu32)))
        ==> (status == SUCCESS
            && attributes == 0u32
            && (config >> 1u32) == 0u32
            && (((config & 1u32) == 1u32) <==> ClockIsEnabled(old_s, clock_id))
            && ((flags & 0xFFu32) != 0u32
                ==> extended_config_val == ClockExtendedConfigValue(old_s, clock_id, flags & 0xFFu32))))
    && (new_s == old_s)
}
