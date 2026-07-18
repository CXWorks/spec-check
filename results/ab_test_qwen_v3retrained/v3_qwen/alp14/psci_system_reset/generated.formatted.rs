pub open spec fn psci_system_reset_spec(result: RmiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result.is_Ok() ==> CurrentRealm(new_s).state == REALM_SYSTEM_OFF)
  && (result.is_Err()
    ==> CurrentRealm(new_s).state == CurrentRealm(old_s).state)
}