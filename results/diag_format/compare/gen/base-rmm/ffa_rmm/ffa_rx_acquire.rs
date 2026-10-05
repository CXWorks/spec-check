pub open spec fn ffa_rx_acquire_spec(result: UInt32, old_s: S, new_s: S) -> bool {
    (!IsImplementedAtInstance(old_s, FFA_RX_ACQUIRE) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsBufferPairRegistered(old_s, vm_id(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!CanRelinquishRxBufferOwnership(old_s, vm_id(old_s)) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, FFA_SUCCESS) ==> RxBufferOwner(new_s, vm_id(old_s)) == Caller)
}