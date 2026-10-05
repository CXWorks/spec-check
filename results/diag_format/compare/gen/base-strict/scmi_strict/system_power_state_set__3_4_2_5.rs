pub open spec fn system_power_state_set__3_4_2_5_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!IsValidSystemPowerState(old_s.system_power_state_set.system_state) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsSystemPowerStateSupportedForAgent(old_s.system_power_state_set.system_state, old_s.system_power_state_set.caller_agent) ==> ResultEqual(result, NOT_SUPPORTED))
    && (old_s.system_power_state_set.system_state == 0x4 && (exists|cpu: ApplicationProcessor| cpu != old_s.system_power_state_set.caller_cpu && (CpuIsRunning(cpu) || CpuIsIdle(cpu)))) ==> ResultEqual(result, DENIED)
    && (old_s.system_power_state_set.system_state == 0x3 ==> SystemPowerStateRequested(old_s.system_power_state_set.system_state))
    && (old_s.system_power_state_set.system_state != 0x3 && Bits(old_s.system_power_state_set.flags, 0, 0) == 1 ==> GracefulSystemPowerStateRequested(old_s.system_power_state_set.system_state))
    && (old_s.system_power_state_set.system_state != 0x3 && Bits(old_s.system_power_state_set.flags, 0, 0) == 0 ==> ForcefulSystemPowerStateRequested(old_s.system_power_state_set.system_state))
}