pub open spec fn cpu_off_spec(calling_cpu: Cpu, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (IsTrustedOsResidentCore(old_s, calling_cpu) ==> result == DENIED)
  && (result == PSCI_SUCCESS ==> CorePoweredDown(new_s, calling_cpu))
  && (result == PSCI_SUCCESS ==> !CallReturns(new_s, calling_cpu))
  && (result == PSCI_SUCCESS ==> (forall (n: TopologyNode), NodePoweredDown(new_s, n) ==> CachesCleaned(new_s, n)))
  && (result == PSCI_SUCCESS ==> !CoreInCoherency(new_s, calling_cpu))
  && (result == PSCI_SUCCESS ==> (forall (n: TopologyNode), (IsCluster(new_s, n) && NodePoweredDown(new_s, n)) ==> !ClusterInCoherency(new_s, n)))
  && (result == PSCI_SUCCESS ==> (forall (n: TopologyNode), (ContainsCore(new_s, n, calling_cpu) && (forall (c: Core), ContainsCore(new_s, n, c) ==> (CalledCpuOff(new_s, c) || RequestedPowerdownAtOrAbove(new_s, c, PowerState(new_s, n))))) ==> NodePoweredDown(new_s, n)))
  && ((!IsTrustedOsResidentCore(old_s, calling_cpu))
    ==> result == PSCI_SUCCESS)
  && (result != PSCI_SUCCESS
    ==> CorePowerState(new_s, calling_cpu) == CorePowerState(old_s, calling_cpu))
  && (result != PSCI_SUCCESS
    ==> CacheState(new_s, calling_cpu) == CacheState(old_s, calling_cpu))
  && (result != PSCI_SUCCESS
    ==> CoherencyState(new_s, calling_cpu) == CoherencyState(old_s, calling_cpu))
  && (result != PSCI_SUCCESS
    ==> NodePowerState(new_s, AncestorNodes(new_s, calling_cpu)) == NodePowerState(old_s, AncestorNodes(old_s, calling_cpu)))
  && ((!(IsTrustedOsResidentCore(old_s, calling_cpu)))
    ==> result == PSCI_SUCCESS)
}