pub open spec fn psci_system_off_spec(old_s: S, new_s: S) -> bool {
  (CurrentRealm(new_s).state == REALM_SYSTEM_OFF)
}
