pub open spec fn ffa_rx_acquire_spec(result: u32, error_code: i32, old_s: S, new_s: S) -> bool {
    (!IsImplementedAtInstance(FFA_RX_ACQUIRE) ==> ResultEqual(result, FFA_ERROR) && ResultEqual(error_code, NOT_SUPPORTED))
    && (!IsBufferPairRegistered(old_s, vm_id(old_s)) ==> ResultEqual(result, FFA_ERROR) && ResultEqual(error_code, INVALID_PARAMETERS))
    && (!CalleeCanRelinquishRxBuffer(old_s, vm_id(old_s)) ==> ResultEqual(result, FFA_ERROR) && ResultEqual(error_code, DENIED))
    && (ResultEqual(result, FFA_SUCCESS) ==> RxBufferOwnedByCaller(new_s, vm_id(old_s)))
}