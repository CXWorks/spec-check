pub open spec fn migrate_spec(target_cpu: Mpidr, fid: UInt, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (!MigrateIsImplemented(old_s) || !MigrationIsRequired(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidMpidr(old_s, target_cpu) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (CallerRegisterWidth(old_s) != FidRegisterWidth(old_s, fid) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (TrustedOsIsUp(old_s) && !TrustedOsIsMigrateCapable(old_s) ==> ResultEqual(result, DENIED))
  && (MigrateImplementationDefinedFailure(old_s, target_cpu) ==> ResultEqual(result, INTERNAL_FAILURE))
  && (CurrentCpu(old_s) != TrustedOsResidentCore(old_s) ==> ResultEqual(result, NOT_PRESENT))
  && (result == RSI_SUCCESS ==> TrustedOsResidentCore(new_s) == target_cpu)
  && ((MigrateIsImplemented(old_s) &&
       IsValidMpidr(old_s, target_cpu) &&
       !(CallerRegisterWidth(old_s) != FidRegisterWidth(old_s, fid)) &&
       !(TrustedOsIsUp(old_s) && !TrustedOsIsMigrateCapable(old_s)) &&
       !MigrateImplementationDefinedFailure(old_s, target_cpu) &&
       !(CurrentCpu(old_s) != TrustedOsResidentCore(old_s)))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> TrustedOsResidentCore(new_s) == TrustedOsResidentCore(old_s))
}