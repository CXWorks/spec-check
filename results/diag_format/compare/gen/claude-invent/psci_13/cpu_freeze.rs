pub open spec fn cpu_freeze_spec(result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (!CpuFreezeImplemented(old_s) ==> (result == NOT_SUPPORTED && new_s == old_s))
    && ((CpuFreezeImplemented(old_s) && CpuOffDenied(old_s)) ==> (result == DENIED && new_s == old_s))
    && ((CpuFreezeImplemented(old_s) && !CpuOffDenied(old_s)) ==> (result != NOT_SUPPORTED && result != DENIED))
}
