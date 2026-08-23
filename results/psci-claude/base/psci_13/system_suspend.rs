pub open spec fn system_suspend_spec(result: int, entry_point_address: Address, context_id: Bits64, old_s: S, new_s: S) -> bool {
    (!AddrIsNonSecure(old_s, entry_point_address) ==> result == PSCI_INVALID_ADDRESS)
    && (!CallerIsLastCpu(old_s) ==> result == PSCI_DENIED)
    && ((AddrIsNonSecure(old_s, entry_point_address) && CallerIsLastCpu(old_s))
        ==> result == PSCI_SUCCESS)
}