pub open spec fn migrate_info_up_cpu_spec(result: UInt64, old_s: S, new_s: S) -> bool {
    (!IsMigrateInfoTypeImplemented() ==> ResultEqual(result, NOT_SUPPORTED))
    && ((PlatformMigrateInfoType() == 0 || PlatformMigrateInfoType() == 1) ==> result == TrustedOsResidentCpu())
    && ((PlatformMigrateInfoType() == 0 || PlatformMigrateInfoType() == 1) ==> IsTargetCpuFormat(result))
    && (PlatformMigrateInfoType() == 2 ==> IsUndefinedValue(result))
    && (old_s == new_s)
}