pub open spec fn PSCI_SYSTEM_OFF_spec(s: S, realm: RmmRealm) -> bool {
    realm.state == RmmRealmState::REALM_SYSTEM_OFF
}