pub open spec fn cpu_off_spec(result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (IsTrustedOsResident(old_s, CallingCore()) ==> ResultEqual(result, DENIED))
    && (!ReturnsToCaller())
    && (PowerState(old_s, CallingCore()) == POWERED_DOWN ==> PowerState(new_s, CallingCore()) == POWERED_DOWN)
    && (Caches(old_s, CallingCore()) == CachesClean ==> Caches(new_s, CallingCore()) == CachesClean)
    && (!IsCoherent(old_s, CallingCore()) ==> !IsCoherent(new_s, CallingCore()))
    && (forall node: Node. (AllCoresCalledCpuOff(old_s, node) || RemainingCoresSuspendedAtOrAbove(old_s, node.level)) ==> PowerState(new_s, node) == POWERED_DOWN)
    && (forall node: Node. PowerState(old_s, node) == POWERED_DOWN ==> PowerState(new_s, node) == POWERED_DOWN)
    && (forall node: Node. PowerState(old_s, node) == POWERED_DOWN ==> Caches(new_s, node) == CachesClean)
    && (forall node: Node. PowerState(old_s, node) == POWERED_DOWN ==> !IsCoherent(new_s, node))
}