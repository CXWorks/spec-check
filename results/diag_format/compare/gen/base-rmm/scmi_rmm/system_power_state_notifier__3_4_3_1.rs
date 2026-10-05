pub open spec fn system_power_state_notifier__3_4_3_1_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS)
    && (IsRegisteredForSystemPowerStateNotify(old_s, agent) ==> NotificationSentToAgent(new_s, agent, SYSTEM_POWER_STATE_NOTIFIER))
    && (flags[31:1] == 0)
    && (!(system_state >= 0x5 && system_state <= 0x7FFFFFFF))
    && (!PlatformImposesShutdownTimeout() ==> (timeout == 0))
}