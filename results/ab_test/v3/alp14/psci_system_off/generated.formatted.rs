pub open spec fn psci_system_off_spec(old_s: S, new_s: S) -> bool {
    new_s.realm.state == RmmRealmState::REALM_SYSTEM_OFF
}