pub open spec fn cpu_default_suspend_spec(result: PsciReturnCode, entry_point_address: UInt64, context_id: UInt64, old_s: S, new_s: S) -> bool {
    (EntryPointAddressIsInvalid(old_s, entry_point_address) ==> result == PSCI_INVALID_ADDRESS)
    && (!EntryPointAddressIsInvalid(old_s, entry_point_address) ==> result == PSCI_SUCCESS)
}
