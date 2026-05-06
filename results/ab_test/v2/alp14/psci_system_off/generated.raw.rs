```verus
pub open spec fn psci_system_off_spec(old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    new_s.realm(realm).state == RmmRealmState::REALM_SYSTEM_OFF
}
```