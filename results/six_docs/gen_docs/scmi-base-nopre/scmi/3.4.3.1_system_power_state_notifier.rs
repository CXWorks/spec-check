pub open spec fn 3.4.3.1_system_power_state_notifier_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS ==> (
        (old_s.system_power_state_notifier_registered == true)
        && (new_s.system_power_state_notifier_registered == old_s.system_power_state_notifier_registered)
        && (new_s.system_power_state_notifier_agent_id == old_s.system_power_state_notifier_agent_id)
        && (new_s.system_power_state_notifier_flags == old_s.system_power_state_notifier_flags)
        && (new_s.system_power_state_notifier_system_state == old_s.system_power_state_notifier_system_state)
        && (new_s.system_power_state_notifier_timeout == old_s.system_power_state_notifier_timeout)
    ))
    && (result == RSI_ERROR_INPUT ==> (
        (old_s.system_power_state_notifier_registered == false)
        || (new_s.system_power_state_notifier_flags & 0xFFFFFFFE != 0)
        || (new_s.system_power_state_notifier_system_state >= 0x80000000 && new_s.system_power_state_notifier_system_state < 0x80000000)
        || (new_s.system_power_state_notifier_system_state == 0 && (new_s.system_power_state_notifier_flags & 0x1) == 0 && new_s.system_power_state_notifier_timeout != 0)
    ))
    && (result != RSI_SUCCESS && result != RSI_ERROR_INPUT ==> true)
}