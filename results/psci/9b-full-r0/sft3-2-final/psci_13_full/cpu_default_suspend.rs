pub open spec fn cpu_default_suspend_spec(entry_point_address: Address, result: int, old_s: S, new_s: S) -> bool {
  (AddrIsNonSecure(old_s, entry_point_address) ==> result == PSCI_SUCCESS || result == PSCI_INVALID_ADDRESS)
  && ((!(AddrIsNonSecure(old_s, entry_point_address)))
    ==> result == PSCI_SUCCESS)
}