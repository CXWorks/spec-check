```verus
pub open spec fn PSCI_SYSTEM_RESET_spec(old_s: S, new_s: S) -> bool {
    let realm = old_s.current_realm();
    new_s.realm_state(realm) == RmmRealmState::REALM_SYSTEM_OFF
}
```