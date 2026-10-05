pub open spec fn system_power_state_set__3_4_2_5_spec(flags: UInt32, system_state: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidSystemPowerState(old_s, system_state) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!IsSystemPowerStateSupportedForAgent(old_s, system_state, caller) ==> ResultEqual(status, NOT_SUPPORTED))
  && (system_state == 4 && OtherApplicationProcessorsRunningOrIdle(old_s, caller) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, OK) ==> SystemPowerState(new_s) == system_state)
  && ((IsValidSystemPowerState(old_s, system_state) &&
       IsSystemPowerStateSupportedForAgent(old_s, system_state, caller) &&
       !(system_state == 4 && OtherApplicationProcessorsRunningOrIdle(old_s, caller)))
    ==> ResultEqual(status, OK))
  && (result != OK
    ==> SystemPowerState(new_s) == SystemPowerState(old_s))
}