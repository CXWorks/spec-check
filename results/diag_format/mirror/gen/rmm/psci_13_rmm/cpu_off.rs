pub open spec fn cpu_off_spec(old_s: S, new_s: S) -> bool {
  (IsTrustedOsResident(old_s, CallingCore()) ==> ResultEqual(result, DENIED))
  && (!ReturnsToCaller(new_s))
  && (PowerState(new_s, CallingCore()) == POWERED_DOWN)
  && (CachesClean(new_s, CallingCore()))
  && (!IsCoherent(new_s, CallingCore()))
  && ((AllCoresCalledCpuOff(new_s, node) || RemainingCoresSuspendedAtOrAbove(new_s, node.level)) ==> PowerState(new_s, node) == POWERED_DOWN)
  && (PowerState(new_s, node) == POWERED_DOWN ==> CachesClean(new_s, node))
  && (PowerState(new_s, node) == POWERED_DOWN ==> !IsCoherent(new_s, node))
  && ((!(IsTrustedOsResident(old_s, CallingCore())))
    ==> ReturnsToCaller(new_s))
  && ((!(IsTrustedOsResident(old_s, CallingCore())))
    ==> PowerState(new_s, CallingCore()) != POWERED_DOWN)
  && ((!(IsTrustedOsResident(old_s, CallingCore())))
    ==> !(CachesClean(new_s, CallingCore())))
  && ((!(IsTrustedOsResident(old_s, CallingCore())))
    ==> IsCoherent(new_s, CallingCore()))
  && ((!(IsTrustedOsResident(old_s, CallingCore())))
    ==> PowerState(new_s, CallingCore()) != POWERED_DOWN)
  && ((!(IsTrustedOsResident(old_s, CallingCore())))
    ==> PowerState(new_s, node) != POWERED_DOWN)
  && ((!(IsTrustedOsResident(old_s, CallingCore())))
    ==> CachesClean(new_s, node))
  && ((!(IsTrustedOsResident(old_s, CallingCore())))
    ==> IsCoherent(new_s, node))
  && (result == RSI_SUCCESS
    ==> ReturnsToCaller(new_s))
  && (result == RSI_SUCCESS
    ==> PowerState(new_s, CallingCore()) == POWERED_DOWN)
  && (result == RSI_SUCCESS
    ==> CachesClean(new_s, CallingCore()))
  && (result == RSI_SUCCESS
    ==> IsCoherent(new_s, CallingCore()))
  && (result == RSI_SUCCESS
    ==> PowerState(new_s, CallingCore()) == POWERED_DOWN)
  && (result == RSI_SUCCESS
    ==> PowerState(new_s, node) == POWERED_DOWN)
  && (result == RSI_SUCCESS
    ==> CachesClean(new_s, node))
  && (result == RSI_SUCCESS
    ==> IsCoherent(new_s, node))
  && (result != RSI_SUCCESS
    ==> PowerState(new_s, CallingCore()) == PowerState(old_s, CallingCore()))
  && (result != RSI_SUCCESS
    ==> Caches(new_s, CallingCore()) == Caches(old_s, CallingCore()))
  && (result != RSI_SUCCESS
    ==> Coherency(new_s, CallingCore()) == Coherency(old_s, CallingCore()))
  && (result != RSI_SUCCESS
    ==> PowerState(new_s, node) == PowerState(old_s, node))
  && (result != RSI_SUCCESS
    ==> Caches(new_s, node) == Caches(old_s, node))
  && (result != RSI_SUCCESS
    ==> Coherency(new_s, node) == Coherency(old_s, node))
}