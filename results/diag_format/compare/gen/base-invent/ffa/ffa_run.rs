pub open spec fn ffa_run_spec(result: int32, old_s: S, new_s: S) -> bool {
    // Failure conditions
    // INVALID_PARAMETERS: Unrecognized endpoint or vCPU ID.
    // Target vCPU is pinned to a different PE.
    (!IsEndpointValid(old_s, TargetInfo(old_s).sp_id) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))
    && (!IsVcpuValid(old_s, TargetInfo(old_s).vcpu_id) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))
    && (IsVcpuPinnedToDifferentPe(old_s, TargetInfo(old_s).vcpu_id) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))
    // NOT_SUPPORTED: This function is not implemented at this FF-A instance.
    (!IsFfaInstanceValid(old_s) ==> ResultEqual(result, FFA_ERROR_NOT_SUPPORTED))
    // DENIED: Callee is not in a state to handle this request.
    // Caller is not allowed to invoke this ABI.
    (!IsCalleeStateValid(old_s, TargetInfo(old_s).vcpu_id) ==> ResultEqual(result, FFA_ERROR_DENIED))
    && (!IsCallerAllowed(old_s) ==> ResultEqual(result, FFA_ERROR_DENIED))
    // BUSY: vCPU is busy and caller must retry later.
    (IsVcpuBusy(old_s, TargetInfo(old_s).vcpu_id) ==> ResultEqual(result, FFA_ERROR_BUSY))
    // ABORTED: vCPU or VM ran into an unexpected error and has aborted.
    (IsVcpuAborted(old_s, TargetInfo(old_s).vcpu_id) ==> ResultEqual(result, FFA_ERROR_ABORTED))
    // NOT_READY: Receiver endpoint is not ready to handle this request.
    (!IsEndpointReady(old_s, TargetInfo(old_s).sp_id) ==> ResultEqual(result, FFA_ERROR_NOT_READY))
    // Success conditions
    // Valid FF-A instances and conduits are listed in Table 14.12.
    (IsFfaInstanceValid(old_s) ==> result == FFA_SUCCESS)
    && (IsCallerAllowed(old_s) ==> result == FFA_SUCCESS)
    && (IsCalleeStateValid(old_s, TargetInfo(old_s).vcpu_id) ==> result == FFA_SUCCESS)
    && (!IsVcpuBusy(old_s, TargetInfo(old_s).vcpu_id) ==> result == FFA_SUCCESS)
    && (!IsVcpuAborted(old_s, TargetInfo(old_s).vcpu_id) ==> result == FFA_SUCCESS)
    && (IsEndpointReady(old_s, TargetInfo(old_s).sp_id) ==> result == FFA_SUCCESS)
    && (IsEndpointValid(old_s, TargetInfo(old_s).sp_id) ==> result == FFA_SUCCESS)
    && (IsVcpuValid(old_s, TargetInfo(old_s).vcpu_id) ==> result == FFA_SUCCESS)
    && (!IsVcpuPinnedToDifferentPe(old_s, TargetInfo(old_s).vcpu_id) ==> result == FFA_SUCCESS)
}