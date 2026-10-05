pub open spec fn psci_features_spec(result: Int32, psci_func_id: UInt32, old_s: S, new_s: S) -> bool {
    (!IsImplementedFunction(psci_func_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (result[31] == 0)
    && (IsCpuSuspendFid(psci_func_id) ==> (result[31:2] == 0 && result[1] == (UsesExtendedStateIdFormat() ? 1 : 0) && result[0] == (SupportsOsInitiatedMode() ? 1 : 0)))
    && (IsSystemOff2Fid(psci_func_id) ==> (result[30:1] == 0 && result[0] == 1))
    && (!IsCpuSuspendFid(psci_func_id) && !IsSystemOff2Fid(psci_func_id) ==> result[31:0] == 0)
    && (old_s == new_s)
}