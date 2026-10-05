pub open spec fn cpu_default_suspend_spec(result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (IsKnownUnavailableAddress(entry_point_address(old_s)) ==> ResultEqual(result, INVALID_ADDRESS))
    && (ReturnedAtNextInstruction(calling_cpu(old_s)) ==> ResultEqual(result, SUCCESS))
    && (ResumedAtEntryPoint(calling_cpu(old_s)) ==> CoreRestartsAtEntryPoint(calling_cpu(old_s), entry_point_address(old_s)))
    && (ResumedAtEntryPoint(calling_cpu(old_s)) ==> ContextIdPresented(calling_cpu(old_s), context_id(old_s)))
    && CacheAndCoherencyManagedByImplementation(calling_cpu(old_s))
    && (AllCoresInDefaultSuspend() ==> !PlatformThermallyCritical())
}