pub open spec fn psci_features_spec(psci_func_id: UInt32, result: Int32, old_s: S, new_s: S) -> bool {
  (!IsImplementedFunction(old_s, psci_func_id) ==> result == NOT_SUPPORTED)
  && (result[31] == 0)
  && (IsCpuSuspendFid(old_s, psci_func_id) ==> result[31:2] == 0)
  && (IsCpuSuspendFid(old_s, psci_func_id) ==> result[1] == (UsesExtendedStateIdFormat() ? 1 : 0))
  && (IsCpuSuspendFid(old_s, psci_func_id) ==> result[0] == (SupportsOsInitiatedMode() ? 1 : 0))
  && (IsSystemOff2Fid(old_s, psci_func_id) ==> result[30:1] == 0)
  && (IsSystemOff2Fid(old_s, psci_func_id) ==> result[0] == 1)
  && (!IsCpuSuspendFid(old_s, psci_func_id) && !IsSystemOff2Fid(old_s, psci_func_id) ==> result[31:0] == 0)
  && ((!(IsImplementedFunction(old_s, psci_func_id)))
    ==> result == NOT_SUPPORTED)
}