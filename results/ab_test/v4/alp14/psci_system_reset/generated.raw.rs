pub open spec fn PSCI_SYSTEM_RESET_spec(old_s: S, new_s: S, realm: RmmRealm) -> bool {
    let fid: u64 = 0xC4000009;
    (
        realm == CurrentRealm(old_s) &&
        new_s.realms[realm].state == REALM_SYSTEM_OFF
    )
}