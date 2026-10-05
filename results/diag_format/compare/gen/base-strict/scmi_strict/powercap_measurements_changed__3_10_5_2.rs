pub open spec fn powercap_measurements_changed__3_10_5_2_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S, recipient_agent: u32, domain_id: u32, agent_id: u32, power: u32, mai: u32) -> bool {
    (result.is_Ok())
    && (IsRegisteredForMeasurementsChangedNotification(old_s, recipient_agent, domain_id))
    && (MaiChanged(old_s, domain_id) || AveragePowerBelowLowerThreshold(old_s, domain_id) || AveragePowerAboveHigherThreshold(old_s, domain_id))
    && (agent_id == 0)
    && (!MaiChanged(old_s, domain_id) ==> power == AveragePowerAtThresholdBreach(old_s, domain_id))
    && (MaiChanged(old_s, domain_id) ==> power == AveragePowerAtMaiChange(old_s, domain_id))
    && (mai == PowerCapMai(old_s, domain_id))
    && (old_s == new_s)
}