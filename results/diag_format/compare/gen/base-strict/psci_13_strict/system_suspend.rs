pub open spec fn system_suspend_spec(result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (!SystemSuspendImplemented(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && (EntryPointKnownInvalid(old_s, entry_point_address) ==> ResultEqual(result, INVALID_ADDRESS))
    && (exists|c: Core| c != CallingCore() && AffinityState(c) != OFF ==> ResultEqual(result, DENIED))
    && (result == PSCI_SUCCESS ==> SystemInDeepestPowerdownState(new_s))
    && (result == PSCI_SUCCESS ==> CoreResumesAtEntryPoint(CallingCore(), entry_point_address, context_id, new_s))
}