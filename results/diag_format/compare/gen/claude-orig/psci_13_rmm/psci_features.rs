pub open spec fn psci_features_spec(fid: UInt32, psci_func_id: UInt32, result: Int32, old_s: S, new_s: S) -> bool {
    (!IsImplementedFunction(psci_func_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (IsImplementedFunction(psci_func_id) ==> (
        (((result as u32) >> 31u32) & 1u32) == 0u32
        && (IsCpuSuspendFid(psci_func_id) ==> (
            ((result as u32) >> 2u32) == 0u32
            && ((((result as u32) >> 1u32) & 1u32) == (if UsesExtendedStateIdFormat() { 1u32 } else { 0u32 }))
            && (((result as u32) & 1u32) == (if SupportsOsInitiatedMode() { 1u32 } else { 0u32 }))
        ))
        && (IsSystemOff2Fid(psci_func_id) ==> (
            (((result as u32) >> 1u32) & 0x3FFF_FFFFu32) == 0u32
            && ((result as u32) & 1u32) == 1u32
        ))
        && ((!IsCpuSuspendFid(psci_func_id) && !IsSystemOff2Fid(psci_func_id)) ==> (result as u32) == 0u32)
    ))
    && new_s == old_s
}
