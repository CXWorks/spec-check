pub open spec fn migrate_info_type_spec(result: MigrateInfoType, old_s: S, new_s: S) -> bool {
    (TrustedOsIsUniprocessor() && TrustedOsIsMigrateCapable() ==> ResultEqual(result, 0))
    && (TrustedOsIsUniprocessor() && !TrustedOsIsMigrateCapable() ==> ResultEqual(result, 1))
    && (TrustedOsIsMultiprocessorAware() || !TrustedOsIsPresent() ==> ResultEqual(result, 2))
    && (ResultEqual(result, 0) ==> ReturnOf(CPU_OFF, on TrustedOsResidentCpu()) == DENIED)
    && (ResultEqual(result, 0) && IsValidMpidr(target_cpu) ==> ReturnOf(MIGRATE(target_cpu), on TrustedOsResidentCpu()) == SUCCESS)
    && (ResultEqual(result, 1) ==> ReturnOf(CPU_OFF, on TrustedOsResidentCpu()) == DENIED)
    && (ResultEqual(result, 1) ==> ReturnOf(MIGRATE(target_cpu)) == DENIED)
    && (ResultEqual(result, 2) || ResultEqual(result, NOT_SUPPORTED) ==> CPU_OFF on any core does not return (no prior MIGRATE needed))
    && (ResultEqual(result, 2) || ResultEqual(result, NOT_SUPPORTED) ==> ReturnOf(MIGRATE(target_cpu)) == NOT_SUPPORTED, with no other effect)
    && (ResultEqual(result, 0) ==> result == PreviousMigrateInfoTypeResult())
    && (ResultEqual(result, 1) ==> result == PreviousMigrateInfoTypeResult())
    && (ResultEqual(result, 2) ==> result == PreviousMigrateInfoTypeResult())
    && (ResultEqual(result, NOT_SUPPORTED) ==> result == PreviousMigrateInfoTypeResult())
    && (old_s == new_s)
}

pub open spec fn migrate_info_up_cpu_spec(result: Mpidr, old_s: S, new_s: S) -> bool {
    (MigrateInfoType() == 0 || MigrateInfoType() == 1 ==> result == MpidrOf(TrustedOsResidentCpu()))
    && (MigrateInfoType() == 2 ==> result is UNDEFINED)
    && (old_s == new_s)
}