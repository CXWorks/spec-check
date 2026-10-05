pub open spec fn system_power_state_notifier__3_4_3_1_spec(agent_id: UInt32, flags: UInt32, system_state: UInt32, timeout: UInt32, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (IsRegisteredForSystemPowerStateNotify(old_s, agent_id) ==> NotificationSentToAgent(new_s, agent_id, SYSTEM_POWER_STATE_NOTIFIER))
  && (flags[31..1] == 0)
  && (!(system_state >= 0x5 && system_state <= 0x7FFFFFFF))
  && (!PlatformImposesShutdownTimeout(old_s) ==> timeout == 0)
  && ((!(IsRegisteredForSystemPowerStateNotify(old_s, agent_id)))
    ==> !(NotificationSentToAgent(new_s, agent_id, SYSTEM_POWER_STATE_NOTIFIER)))
  && (result == RSI_SUCCESS
    ==> flags[31..1] == 0)
  && (result == RSI_SUCCESS
    ==> !(system_state >= 0x5 && system_state <= 0x7FFFFFFF))
  && (result == RSI_SUCCESS
    ==> !PlatformImposesShutdownTimeout(old_s) ==> timeout == 0)
}