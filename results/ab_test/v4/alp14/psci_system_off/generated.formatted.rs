pub open spec fn PSCI_SYSTEM_OFF_spec(old_s: S, new_s: S, realm: RmmRealm) -> bool {
    let old_realm = old_s.realm(realm) in
    let new_realm = new_s.realm(realm) in
    old_realm.state != RmmRealmState::REALM_SYSTEM_OFF ==>
        new_realm.state == RmmRealmState::REALM_SYSTEM_OFF
}