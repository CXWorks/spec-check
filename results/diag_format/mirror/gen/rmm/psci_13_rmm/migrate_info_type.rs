pub open spec fn migrate_info_type_spec(result: MigrateInfoType, old_s: S, new_s: S) -> bool {
  (TrustedOsIsUniprocessor(old_s) && TrustedOsIsMigrateCapable(old_s) ==> result == 0)
  && (TrustedOsIsUniprocessor(old_s) && !TrustedOsIsMigrateCapable(old_s) ==> result == 1)
  && (TrustedOsIsMultiprocessorAware(old_s) || !TrustedOsIsPresent(old_s) ==> result == 2)
  && (result == PreviousMigrateInfoTypeResult(old_s))
  && (result == 0 ==> ReturnOf(CPU_OFF, on TrustedOsResidentCpu(old_s)) == DENIED)
  && (result == 0 && IsValidMpidr(old_s, target_cpu) ==> ReturnOf(MIGRATE(target_cpu), on TrustedOsResidentCpu(old_s)) == SUCCESS)
  && (result == 1 ==> ReturnOf(CPU_OFF, on TrustedOsResidentCpu(old_s)) == DENIED)
  && (result == 1 ==> ReturnOf(MIGRATE(target_cpu)) == DENIED)
  && (result == 2 || result == NOT_SUPPORTED ==> CPU_OFF on any core does not return (no prior MIGRATE needed))
  && (result == 2 || result == NOT_SUPPORTED ==> ReturnOf(MIGRATE(target_cpu)) == NOT_SUPPORTED, with no other effect)
  && ((!(TrustedOsIsUniprocessor(old_s) && TrustedOsIsMigrateCapable(old_s)) &&
       !(TrustedOsIsUniprocessor(old_s) && !TrustedOsIsMigrateCapable(old_s)) &&
       !(TrustedOsIsMultiprocessorAware(old_s) || !TrustedOsIsPresent(old_s)))
    ==> result == 2)
}