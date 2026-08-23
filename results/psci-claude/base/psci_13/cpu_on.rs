pub open spec fn cpu_on_spec(result: int, target_cpu: Bits64, entry_point_address: Address, context_id: Bits64, old_s: S, new_s: S) -> bool {
    (!CpuIsValid(old_s, target_cpu) ==> result == PSCI_INVALID_PARAMETERS)
    && ((CpuIsValid(old_s, target_cpu) && !AddrIsNonSecure(old_s, entry_point_address)) ==> result == PSCI_INVALID_ADDRESS)
    && ((CpuIsValid(old_s, target_cpu) && AddrIsNonSecure(old_s, entry_point_address) && CpuIsOn(old_s, target_cpu)) ==> result == PSCI_ALREADY_ON)
    && ((CpuIsValid(old_s, target_cpu) && AddrIsNonSecure(old_s, entry_point_address) && !CpuIsOn(old_s, target_cpu) && CpuIsOnPending(old_s, target_cpu)) ==> result == PSCI_ON_PENDING)
    && ((CpuIsValid(old_s, target_cpu)
        && AddrIsNonSecure(old_s, entry_point_address)
        && !CpuIsOn(old_s, target_cpu)
        && !CpuIsOnPending(old_s, target_cpu))
        ==> (result == PSCI_SUCCESS
             || result == PSCI_INTERNAL_FAILURE
             || result == PSCI_DENIED))
    && ((CpuIsValid(old_s, target_cpu)
        && AddrIsNonSecure(old_s, entry_point_address)
        && !CpuIsOn(old_s, target_cpu)
        && !CpuIsOnPending(old_s, target_cpu)
        && result == PSCI_SUCCESS)
        ==> (CpuIsOn(new_s, target_cpu) || CpuIsOnPending(new_s, target_cpu)))
    && (result != PSCI_SUCCESS ==> new_s == old_s)
}