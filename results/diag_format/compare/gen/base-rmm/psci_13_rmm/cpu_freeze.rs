pub open spec fn cpu_freeze_spec(result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (!IsImplemented(CPU_FREEZE) ==> ResultEqual(result, NOT_SUPPORTED))
    && (CpuOffWouldBeDenied(CurrentCore()) ==> ResultEqual(result, DENIED))
    && (result == NOT_SUPPORTED ==> !ResultEqual(result, DENIED))
    && (result == DENIED ==> !ResultEqual(result, NOT_SUPPORTED))
    && (result != NOT_SUPPORTED && result != DENIED ==> true)
    && (result == NOT_SUPPORTED ==> true)
    && (result == DENIED ==> true)
}