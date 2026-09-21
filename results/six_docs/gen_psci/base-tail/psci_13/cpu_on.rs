pub open spec fn cpu_on_spec(result: int, old_s: S, new_s: S) -> bool {
    (!CpuIsValid(old_s, 0u64 as Bits64) ==> result == PSCI_INVALID_PARAMETERS)
    && (AddrIsNonSecure(old_s, 0u64 as Address) ==> result == PSCI_INVALID_ADDRESS)
    && (CpuIsOn(old_s, 0u64 as Bits64) ==> result == PSCI_ALREADY_ON)
    && (CpuIsOnPending(old_s, 0u64 as Bits64) ==> result == PSCI_ON_PENDING)
    && (result == PSCI_INTERNAL_FAILURE ==> true)
    && (result == PSCI_DENIED ==> true)
    && (result == PSCI_SUCCESS ==> CpuIsOn(new_s, 0u64 as Bits64))
    && (result == PSCI_SUCCESS ==> !CpuIsOnPending(new_s, 0u64 as Bits64))
}