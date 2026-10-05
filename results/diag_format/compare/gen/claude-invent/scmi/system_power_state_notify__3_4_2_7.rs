pub open spec fn system_power_state_notify__3_4_2_7_spec(status: i32, agent_id: u32, notify_enable: u32, old_s: S, new_s: S) -> bool {
    ((notify_enable & 0xFFFF_FFFEu32) != 0u32 ==> status != SUCCESS)
    && (!SystemPowerStateNotifySupported(old_s, agent_id) ==> status != SUCCESS)
    && (status == NOT_SUPPORTED ==> !SystemPowerStateNotifySupported(old_s, agent_id))
    && (status == INVALID_PARAMETERS ==> (notify_enable & 0xFFFF_FFFEu32) != 0u32)
    && (status != SUCCESS ==> new_s == old_s)
    && ((SystemPowerStateNotifySupported(old_s, agent_id) && (notify_enable & 0xFFFF_FFFEu32) == 0u32) ==> status == SUCCESS)
    && (status == SUCCESS ==> (
        SystemPowerStateNotifyEnabled(new_s, agent_id) == ((notify_enable & 1u32) == 1u32)
        && SystemPowerStateNotifyOthersUnchanged(old_s, new_s, agent_id)
    ))
}
