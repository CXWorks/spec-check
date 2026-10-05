pub open spec fn system_suspend_spec(result: PsciReturnCode, entry_point_address: UInt64, context_id: UInt64, old_s: S, new_s: S) -> bool {
    (!SystemSuspendImplemented(old_s) ==> result == PSCI_NOT_SUPPORTED)
    && ((SystemSuspendImplemented(old_s) && EntryPointAddressKnownInvalid(old_s, entry_point_address))
        ==> (result == PSCI_INVALID_ADDRESS
             || (OtherCoreNotOff(old_s) && result == PSCI_DENIED)))
    && ((SystemSuspendImplemented(old_s) && OtherCoreNotOff(old_s))
        ==> (result == PSCI_DENIED
             || (EntryPointAddressKnownInvalid(old_s, entry_point_address) && result == PSCI_INVALID_ADDRESS)))
    && ((result == PSCI_NOT_SUPPORTED || result == PSCI_INVALID_ADDRESS || result == PSCI_DENIED) ==> new_s == old_s)
}
