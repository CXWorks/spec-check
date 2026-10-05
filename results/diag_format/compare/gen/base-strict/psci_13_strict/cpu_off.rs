pub open spec fn cpu_off_spec(result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (IsTrustedOsResidentCore(old_s, calling_cpu) ==> ResultEqual(result, DENIED))
    && (CorePoweredDown(new_s, calling_cpu))
    && (!CallReturns(new_s, calling_cpu))
    && (forall|n: TopologyNode| NodePoweredDown(n) ==> CachesCleaned(n))
    && (!CoreInCoherency(new_s, calling_cpu))
    && (forall|n: TopologyNode| (IsCluster(n) && NodePoweredDown(n)) ==> !ClusterInCoherency(n))
    && (forall|n: TopologyNode| (ContainsCore(n, calling_cpu) && forall|c: Core| ContainsCore(n, c) ==> (CalledCpuOff(c) || RequestedPowerdownAtOrAbove(c, PowerLevel(n)))) ==> NodePoweredDown(n))
}