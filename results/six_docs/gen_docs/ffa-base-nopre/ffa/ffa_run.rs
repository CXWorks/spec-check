pub open spec fn ffa_run_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == FFA_ERROR_INVALID_PARAMETERS ==> (
        !IsEndpointValid(old_s, TargetInfo(old_s).sp_id)
        || !IsEndpointValid(old_s, TargetInfo(old_s).vcpu_id)
        || IsVcpuPinnedToDifferentPe(old_s, TargetInfo(old_s).sp_id, TargetInfo(old_s).vcpu_id)
    ))
    && (result == FFA_ERROR_NOT_SUPPORTED ==> !IsFfaRunSupported(old_s))
    && (result == FFA_ERROR_DENIED ==> (
        !IsCalleeInStateToHandleRequest(old_s, TargetInfo(old_s).sp_id, TargetInfo(old_s).vcpu_id)
        || !IsCallerAllowedToInvokeAbi(old_s)
    ))
    && (result == FFA_ERROR_BUSY ==> IsVcpuBusy(old_s, TargetInfo(old_s).sp_id, TargetInfo(old_s).vcpu_id))
    && (result == FFA_ERROR_ABORTED ==> (
        IsVcpuAborted(old_s, TargetInfo(old_s).sp_id, TargetInfo(old_s).vcpu_id)
        || IsVmAborted(old_s, TargetInfo(old_s).sp_id)
    ))
    && (result == FFA_ERROR_NOT_READY ==> !IsReceiverEndpointReady(old_s, TargetInfo(old_s).sp_id, TargetInfo(old_s).vcpu_id))
    && (result == FFA_SUCCESS ==> (
        IsEndpointValid(old_s, TargetInfo(old_s).sp_id)
        && IsEndpointValid(old_s, TargetInfo(old_s).vcpu_id)
        && !IsVcpuPinnedToDifferentPe(old_s, TargetInfo(old_s).sp_id, TargetInfo(old_s).vcpu_id)
        && IsFfaRunSupported(old_s)
        && IsCallerAllowedToInvokeAbi(old_s)
        && IsCalleeInStateToHandleRequest(old_s, TargetInfo(old_s).sp_id, TargetInfo(old_s).vcpu_id)
        && !IsVcpuBusy(old_s, TargetInfo(old_s).sp_id, TargetInfo(old_s).vcpu_id)
        && !IsVcpuAborted(old_s, TargetInfo(old_s).sp_id, TargetInfo(old_s).vcpu_id)
        && !IsVmAborted(old_s, TargetInfo(old_s).sp_id)
        && IsReceiverEndpointReady(old_s, TargetInfo(old_s).sp_id, TargetInfo(old_s).vcpu_id)
    ))
}