pub open spec fn powercap_cap_changed__3_10_5_1_spec(agent_id: UInt32, domain_id: UInt32, power_cap: UInt32, cai: UInt32, cpli: UInt32, old_s: S, new_s: S) -> bool {
  (IsRegisteredForPowerCapNotification(old_s, recipient, domain_id) ==> NotificationSent(new_s, recipient, POWERCAP_CAP_CHANGED))
  && (PowerCapChanged(old_s, domain_id) || CaiChanged(old_s, domain_id) ==> NotificationSent(new_s, recipient, POWERCAP_CAP_CHANGED))
  && (TransitionCompleted(new_s, domain_id))
  && (power_cap == DomainPowerCap(new_s, domain_id))
  && (cai == DomainCai(new_s, domain_id))
  && (!DomainSupportsCpc(old_s, domain_id) ==> cpli == 0)
  && ((!(IsRegisteredForPowerCapNotification(old_s, recipient, domain_id)) &&
       !(PowerCapChanged(old_s, domain_id) || CaiChanged(old_s, domain_id)))
    ==> TransitionCompleted(new_s, domain_id))
  && (DomainSupportsCpc(old_s, domain_id) ==> cpli == 0)
}