pub open spec fn clock_attributes__3_6_2_5_spec(result: Int32, attributes: UInt32, clock_name: UInt8[16], clock_enable_delay: UInt32, old_s: S, new_s: S) -> bool {
    (!IsValidClockId(clock_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (IsValidClockId(clock_id(old_s)) ==> (
        ResultEqual(result, SUCCESS)
        && (attributes[31] as int) == (ClockSupportsRateChangeNotifications(clock_id(old_s)) ? 1 : 0)
        && (attributes[30] as int) == (ClockSupportsRateChangeRequestedNotifications(clock_id(old_s)) ? 1 : 0)
        && (attributes[29] as int) == (ClockNameLength(clock_id(old_s)) > 16 ? 1 : 0)
        && (attributes[28] as int) == (ClockAdvertisesParentIds(clock_id(old_s)) ? 1 : 0)
        && (attributes[27] as int) == (ClockSupportsExtendedConfig(clock_id(old_s)) ? 1 : 0)
        && (attributes[26:2] as int) == 0
        && (attributes[1] as int) == (ClockHasRestrictions(clock_id(old_s)) && IsClockGetPermissionsImplemented() ? 1 : 0)
        && (attributes[0] as int) == (ClockIsEnabled(clock_id(old_s)) ? 1 : 0)
        && clock_name == ClockNameString(clock_id(old_s))
        && clock_enable_delay == ClockWorstCaseEnableDelayUs(clock_id(old_s))
    ))
}