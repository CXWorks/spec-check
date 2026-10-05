pub open spec fn cpu_freeze_spec(fid: UInt64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (!IsImplemented(CPU_FREEZE) ==> ResultEqual(result, NOT_SUPPORTED))
    && ((IsImplemented(CPU_FREEZE) && CpuOffWouldBeDenied(old_s, CurrentCore(old_s))) ==> ResultEqual(result, DENIED))
    && ((IsImplemented(CPU_FREEZE) && !CpuOffWouldBeDenied(old_s, CurrentCore(old_s))) ==> (CoreAt(new_s, CurrentCore(old_s)).power_state == IMPLEMENTATION_DEFINED_LOW_POWER_STATE))
}
