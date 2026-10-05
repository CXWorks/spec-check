pub open spec fn clock_attributes__3_6_2_5_spec(clock_id: u32, status: i32, attributes: u32, clock_name: Seq<u8>, clock_enable_delay: u32, old_s: S, new_s: S) -> bool {
    (!ClockIsValidForAgent(old_s, clock_id) ==> status == -4i32)
    && (status == 0i32 ==> (
        ClockIsValidForAgent(old_s, clock_id)
        && (attributes & 0x07FF_FFFCu32) == 0u32
        && (((attributes >> 31u32) & 1u32) == 1u32) == ClockRateChangeNotificationsSupported(old_s, clock_id)
        && (((attributes >> 30u32) & 1u32) == 1u32) == ClockRateChangeRequestedNotificationsSupported(old_s, clock_id)
        && (((attributes >> 29u32) & 1u32) == 1u32) == (ClockNameLength(old_s, clock_id) > 16)
        && (((attributes >> 28u32) & 1u32) == 1u32) == ClockParentIdentifiersAdvertised(old_s, clock_id)
        && (((attributes >> 27u32) & 1u32) == 1u32) == ClockExtendedConfigSupported(old_s, clock_id)
        && (((attributes >> 1u32) & 1u32) == 1u32) == ClockHasDiscoverableRestrictions(old_s, clock_id)
        && ((attributes & 1u32) == 1u32) == ClockIsEnabled(old_s, clock_id)
        && clock_name.len() == 16
        && (exists|i: int| 0 <= i < 16 && clock_name[i] == 0u8)
        && (clock_enable_delay != 0u32 ==> clock_enable_delay as int == ClockWorstCaseEnableDelayUs(old_s, clock_id))
    ))
    && new_s == old_s
}
