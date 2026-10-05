pub open spec fn power_state_change_requested__3_3_3_2_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS ==> (IsRegisteredForPowerStateChangeRequested(old_s, recipient_agent) && agent_id != recipient_agent && PlatformReceivedPowerStateChangeRequest(old_s, agent_id, domain_id, power_state)))
}