pub open spec fn psci_system_off_spec(fid: UInt64, old_s: S, new_s: S) -> bool {
    (fid == 0x84000008 ==> CurrentRealm(new_s).state == REALM_SYSTEM_OFF)
}