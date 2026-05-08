```verus
pub open spec fn PSCI_SYSTEM_OFF_spec(old_s: S, new_s: S) -> bool {
    let realm = old_s.realm;
    new_s.realm.state == RmmRealmState::REALM_SYSTEM_OFF
}
```