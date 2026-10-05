pub open spec fn system_power_state_set__3_4_2_5_spec(flags: UInt32, system_state: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidSystemPowerState(old_s, system_state) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!IsSystemPowerStateSupportedForAgent(old_s, system_state, caller_agent) ==> ResultEqual(status, NOT_SUPPORTED))
  && (system_state == 0x4 && (exists|cpu: ApplicationProcessor| cpu != caller_cpu && (CpuIsRunning(cpu) || CpuIsIdle(cpu))) ==> ResultEqual(status, DENIED))
  && (result == RSI_SUCCESS && system_state == 0x3 ==> SystemPowerStateRequested(new_s, system_state))
  && (result == RSI_SUCCESS && system_state != 0x3 && Bits(flags, 0, 0) == 1 ==> GracefulSystemPowerStateRequested(new_s, system_state))
  && (result == RSI_SUCCESS && system_state != 0x3 && Bits(flags, 0, 0) == 0 ==> ForcefulSystemPowerStateRequested(new_s, system_state))
  && ((IsValidSystemPowerState(old_s, system_state) &&
       IsSystemPowerStateSupportedForAgent(old_s, system_state, caller_agent) &&
       !(system_state == 0x4 && (exists|cpu: ApplicationProcessor| cpu != caller_cpu && (CpuIsRunning(cpu) || CpuIsIdle(cpu))))))
    ==> status == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> SystemPowerStateRequested(new_s, system_state) == SystemPowerStateRequested(old_s, system_state))
  && (result != RSI_SUCCESS
    ==> GracefulSystemPowerStateRequested(new_s, system_state) == GracefulSystemPowerStateRequested(old_s, system_state))
  && (result != RSI_SUCCESS
    ==> ForcefulSystemPowerStateRequested(new_s, system_state) == ForcefulSystemPowerStateRequested(old_s, system_state))
}