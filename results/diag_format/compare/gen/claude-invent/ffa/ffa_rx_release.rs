pub open spec fn ffa_rx_release_spec(result: FfaStatus, old_s: S, new_s: S, function_id: UInt32, vm_id: UInt32) -> bool {
    (!FfaRxReleaseImplemented(old_s) ==> (result == NOT_SUPPORTED && new_s == old_s))
    && ((FfaRxReleaseImplemented(old_s)
         && IsNonSecurePhysicalInstance(old_s)
         && !BufferPairRegisteredByHypervisorForVm(old_s, vm_id & 0xFFFFu32))
        ==> (result == INVALID_PARAMETERS && new_s == old_s))
    && ((FfaRxReleaseImplemented(old_s)
         && (IsNonSecurePhysicalInstance(old_s) ==> BufferPairRegisteredByHypervisorForVm(old_s, vm_id & 0xFFFFu32))
         && !CallerOwnsRxBuffer(old_s, vm_id & 0xFFFFu32))
        ==> (result == DENIED && new_s == old_s))
    && ((FfaRxReleaseImplemented(old_s)
         && (IsNonSecurePhysicalInstance(old_s) ==> BufferPairRegisteredByHypervisorForVm(old_s, vm_id & 0xFFFFu32))
         && CallerOwnsRxBuffer(old_s, vm_id & 0xFFFFu32))
        ==> (result == FFA_SUCCESS
             && !CallerOwnsRxBuffer(new_s, vm_id & 0xFFFFu32)
             && RxBufferOwnershipRelinquished(old_s, new_s, vm_id & 0xFFFFu32)))
}
