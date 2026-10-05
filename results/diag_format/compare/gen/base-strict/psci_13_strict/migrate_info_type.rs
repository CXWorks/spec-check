pub open spec fn migrate_info_type_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!MigrateInfoTypeImplemented() ==> ResultEqual(result, NOT_SUPPORTED))
    && (result == 0 || result == 1 || result == 2)
    && (result == TrustedOsMigrateInfoType())
    && (result == 0 ==> TrustedOsIsUniprocessor() && TrustedOsIsMigrateCapable())
    && (result == 1 ==> TrustedOsIsUniprocessor() && !TrustedOsIsMigrateCapable())
    && (result == 2 ==> TrustedOsIsMpCapable() || !TrustedOsPresent())
    && (result == 0 ==> MigrateToValidTargetReturns(SUCCESS))
    && (result == 0 ==> CpuOffOnResidentCoreReturns(DENIED))
    && (result == 1 ==> MigrateToValidTargetReturns(DENIED))
    && (result == 1 ==> CpuOffOnResidentCoreReturns(DENIED))
    && (result == 2 ==> MigrateToValidTargetReturns(NOT_SUPPORTED) && MigrateHasNoEffect())
    && (result == 2 ==> CpuOffOnResidentCoreDoesNotReturn())
    && (old_s == new_s)
}

pub open spec fn migrate_info_up_cpu_spec(result: Mpidr, old_s: S, new_s: S) -> bool {
    ((TrustedOsMigrateInfoType() == 0 || TrustedOsMigrateInfoType() == 1) ==> (result == MpidrOf(TrustedOsResidentCore()) && IsTargetCpuFormat(result)))
    && (TrustedOsMigrateInfoType() == 2 ==> IsUndefinedValue(result))
    && (old_s == new_s)
}