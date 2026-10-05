pub open spec fn system_power_state_set__3_4_2_5_spec(flags: UInt32, system_state: UInt32, status: i32, old_s: S, new_s: S) -> bool {
    (((system_state as int) >= 0x5 && (system_state as int) <= 0x7FFFFFFF)
        ==> (status == INVALID_PARAMETERS && new_s == old_s))
    && (((system_state as int) <= 0x4
            && system_state == 0x3
            && CallerIsPsciImplementation(old_s)
            && SystemImplementsOspmSystemView(old_s))
        ==> (status == NOT_SUPPORTED && new_s == old_s))
    && (((system_state as int) <= 0x4
            && !IsSystemPowerStateSupportedForCaller(old_s, system_state))
        ==> (status == NOT_SUPPORTED && new_s == old_s))
    && ((system_state == 0x4
            && IsSystemPowerStateSupportedForCaller(old_s, system_state)
            && OtherApplicationProcessorsRunningOrIdle(old_s))
        ==> (status == DENIED && new_s == old_s))
}
