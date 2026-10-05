pub open spec fn clock_rate_changed__3_6_4_1_spec(agent_id: UInt32, clock_id: UInt32, rate: Seq<UInt32>, old_s: S, new_s: S) -> bool {
    (rate.len() == 2)
    && (rate.len() == 2 ==> (
        (IsValidClockId(old_s, clock_id)
            && ClockRateNotifySubscribed(old_s, clock_id)
            && ClockRateChangeTransitionCompleted(old_s, new_s, clock_id))
        ==> (
            ((rate[0] as int) + (rate[1] as int) * 0x1_0000_0000) == ClockRateHz(new_s, clock_id)
            && ClockRateChangeCausedBy(old_s, new_s, clock_id, agent_id)
            && ClockRateNotifySubscribed(new_s, clock_id) == ClockRateNotifySubscribed(old_s, clock_id)
        )
    ))
}
