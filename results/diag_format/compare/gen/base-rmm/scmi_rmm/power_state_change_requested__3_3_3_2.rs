pub open spec fn power_state_change_requested__3_3_3_2_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (!IsRegisteredForPowerStateChangeRequested(old_s, recipient_agent) ==> result == RSI_ERROR_INPUT)
    && (!PowerStateChangeRequestReceived(old_s, agent_id, domain_id, power_state) ==> result == RSI_ERROR_INPUT)
    && (agent_id == recipient_agent ==> result == RSI_ERROR_INPUT)
    && (result == RSI_SUCCESS ==> NotificationSentTo(new_s, recipient_agent, agent_id, domain_id, power_state))
}