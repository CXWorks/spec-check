pub open spec fn cpu_suspend_spec(result: int, power_state: Bits64, entry_point_address: Address, context_id: Bits64, old_s: S, new_s: S) -> bool {
    (!AddrIsNonSecure(old_s, entry_point_address) ==> result == PSCI_INVALID_ADDRESS)
    && (AddrIsNonSecure(old_s, entry_point_address) ==> result == PSCI_SUCCESS)
}