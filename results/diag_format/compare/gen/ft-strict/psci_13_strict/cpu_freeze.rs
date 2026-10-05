pub open spec fn cpu_freeze_spec(calling_cpu: UInt64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (!IsCpuFreezeImplemented(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (CpuOffWouldReturnDenied(old_s, calling_cpu) ==> ResultEqual(result, DENIED))
  && (result == PSCI_SUCCESS ==> !ReturnsToCaller(new_s, calling_cpu))
  && (result == PSCI_SUCCESS ==> CoreInImplDefinedLowPowerState(new_s, calling_cpu))
  && (result == PSCI_SUCCESS ==> (forall|i: Interrupt| IsWakeupInterruptFor(new_s, i, calling_cpu) ==> (!WakesCore(new_s, i, calling_cpu) && IsPendingOrActive(new_s, i))))
  && (result == PSCI_SUCCESS ==> ResumesOnlyViaCpuOn(new_s, calling_cpu))
  && (result == PSCI_SUCCESS ==> CacheAndCoherencyManagedByImplementation(new_s, calling_cpu))
  && (result == PSCI_SUCCESS ==> AllCoresExceptOneFrozen(new_s) ==> !PlatformThermallyCritical(new_s))
  && ((IsCpuFreezeImplemented(old_s) &&
       !CpuOffWouldReturnDenied(old_s, calling_cpu))
    ==> result == PSCI_SUCCESS)
  && (result != PSCI_SUCCESS
    ==> ReturnsToCaller(new_s, calling_cpu))
  && (result != PSCI_SUCCESS
    ==> !CoreInImplDefinedLowPowerState(new_s, calling_cpu))
  && (result != PSCI_SUCCESS
    ==> (forall|i: Interrupt| IsWakeupInterruptFor(new_s, i, calling_cpu) ==> (WakesCore(new_s, i, calling_cpu) || !IsPendingOrActive(new_s, i))))
  && (result != PSCI_SUCCESS
    ==> !ResumesOnlyViaCpuOn(new_s, calling_cpu))
  && (result != PSCI_SUCCESS
    ==> !CacheAndCoherencyManagedByImplementation(new_s, calling_cpu))
  && (result != PSCI_SUCCESS
    ==> PlatformThermallyCritical(new_s))
}