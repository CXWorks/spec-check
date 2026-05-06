```verus
pub open spec fn psci_system_reset_spec(old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    new_s.realm.state == RmmRealmState::REALM_SYSTEM_OFF
}
```