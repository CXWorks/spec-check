pub open spec fn mem_protect_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!MemProtectImplemented() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!Old(MemProtectEnabled()) ==> ResultEqual(result, 0))
    && (Old(MemProtectEnabled()) ==> ResultEqual(result, 1))
    && (MemProtectEnabled() == (enable != 0))
    && MemProtectCheckRangeImplemented()
    && (MemProtectEnabled() ==> AllCallerAccessibleVolatileMemoryOverwrittenOnNextNonWarmResetBoot())
    && (MemProtectEnabled() ==> CallerMemoryPreservedOnSystemWarmReset())
    && MemProtectDisabledBeforeOsBootLoaderHandover()
}