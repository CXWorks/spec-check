pub open spec fn migrate_spec(target_cpu: UInt64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (!MigrateIsImplemented(old_s) || !MigrationRequired(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidMpidr(old_s, target_cpu) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (CallerRegisterWidth(old_s) != FidRegisterWidth(old_s, 0) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (TrustedOsIsUp(old_s) && !TrustedOsIsMigrateCapable(old_s) ==> ResultEqual(result, DENIED))
  && (CurrentCore(old_s) != TrustedOsResidentCore(old_s) ==> ResultEqual(result, NOT_PRESENT))
  && (MigrateImplementationDefinedFailure(old_s, target_cpu) ==> ResultEqual(result, INTERNAL_FAILURE))
  && (result == PSCI_SUCCESS ==> TrustedOsResidentCore(new_s) == target_cpu)
  && ((MigrateIsImplemented(old_s) &&
       IsValidMpidr(old_s, target_cpu) &&
       !(CallerRegisterWidth(old_s) != FidRegisterWidth(old_s, 0)) &&
       !(TrustedOsIsUp(old_s) && !TrustedOsIsMigrateCapable(old_s)) &&
       !(CurrentCore(old_s) != TrustedOsResidentCore(old_s)) &&
       !(MigrateImplementationDefinedFailure(old_s, target_cpu)))
    ==> result == PSCI_SUCCESS)
  && (result != PSCI_SUCCESS
    ==> TrustedOsResidentCore(new_s) == TrustedOsResidentCore(old_s))
}