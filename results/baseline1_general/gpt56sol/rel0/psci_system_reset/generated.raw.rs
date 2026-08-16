pub open spec fn psci_system_reset_spec(fid: UInt64, old_s: S, new_s: S) -> bool {
    (fid == 0x84000009 ==> CurrentRealm(new_s).state == REALM_SYSTEM_OFF)
}