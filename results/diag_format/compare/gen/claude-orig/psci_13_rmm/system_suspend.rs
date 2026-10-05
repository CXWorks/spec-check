pub open spec fn system_suspend_spec(entry_point_address: Address, context_id: UInt64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (!SystemSuspendImplemented(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && (IsKnownUnavailableToCaller(old_s, entry_point_address) ==> ResultEqual(result, INVALID_ADDRESS))
    && ((exists|core: int| core != CallingCore(old_s) && CoreState(old_s, core) != OFF) ==> ResultEqual(result, DENIED))
    && ((SystemSuspendImplemented(old_s)
        && !IsKnownUnavailableToCaller(old_s, entry_point_address)
        && !(exists|core: int| core != CallingCore(old_s) && CoreState(old_s, core) != OFF))
        ==> (SystemPowerState(new_s) == DeepestPlatformPowerdownState(old_s)))
}
