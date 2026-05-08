pub open spec fn PSCI_SYSTEM_RESET_spec(s: S, realm: RmmRealm) -> bool {
    realm.state == RmmRealmState::REALM_SYSTEM_OFF
}