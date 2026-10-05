pub open spec fn powercap_cap_changed__3_10_5_1_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S, recipient: AgentId, domain_id: UInt32, power_cap: UInt32, cai: UInt32, cpli: UInt32) -> bool {
    (!IsRegisteredForPowerCapNotification(old_s, recipient, domain_id) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PowerCapChanged(old_s, domain_id) && !CaiChanged(old_s, domain_id) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!TransitionCompleted(old_s, domain_id) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (result.is_Ok() ==> NotificationSent(old_s, recipient, POWERCAP_CAP_CHANGED))
    && (result.is_Ok() ==> power_cap == DomainPowerCap(old_s, domain_id))
    && (result.is_Ok() ==> cai == DomainCai(old_s, domain_id))
    && (!DomainSupportsCpc(old_s, domain_id) ==> cpli == 0)
    && (result.is_Ok() ==> cpli == 0)
}