pub open spec fn cpu_off_spec(result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (IsTrustedOsResidentCore(old_s) ==> result == PSCI_DENIED)
    && (!IsTrustedOsResidentCore(old_s) ==> IsCallingCorePoweredDown(new_s))
}
