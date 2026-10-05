pub open spec fn powercap_cap_changed__3_10_5_1_spec(result: (), old_s: S, new_s: S, recipient: AgentId, domain_id: UInt32, agent_id: UInt32, power_cap: UInt32, cai: UInt32, cpli: UInt32) -> bool {
    (AgentRegisteredForCapChangeNotification(old_s, recipient, domain_id) ==> NotificationSent(old_s, recipient, domain_id))
    && (PowerCapTransitionCompleted(old_s, domain_id))
    && (agent_id == AgentCausingChange(old_s, domain_id))
    && (power_cap == DomainPowerCap(old_s, domain_id))
    && (cai == DomainCai(old_s, domain_id))
    && (!DomainSupportsCpc(old_s, domain_id) ==> cpli == 0)
}