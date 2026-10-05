pub open spec fn clock_rate_changed__3_6_4_1_spec(result: RsiCommandReturnCode, old_s: S, new_s: S, recipient_agent: u32, clock_id: u32, agent_id: u32, rate_lo: u32, rate_hi: u32) -> bool {
    (IsRegisteredForClockRateNotification(old_s, recipient_agent, clock_id) ==> result == RSI_SUCCESS)
    && (result == RSI_SUCCESS ==> NotificationSentTo(new_s, recipient_agent))
    && (result == RSI_SUCCESS ==> ClockRateTransitionComplete(new_s, clock_id))
    && (result == RSI_SUCCESS ==> ((rate_hi as u64) << 32 | (rate_lo as u64)) == ClockRate(new_s, clock_id))
    && (result == RSI_SUCCESS ==> agent_id == AgentThatCausedRateChange(new_s, clock_id))
    && (result == RSI_SUCCESS ==> old_s == new_s)
}