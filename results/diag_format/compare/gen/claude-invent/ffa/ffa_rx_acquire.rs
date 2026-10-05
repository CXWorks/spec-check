pub open spec fn ffa_rx_acquire_spec(vm_id: UInt32, result: FfaReturnCode, old_s: S, new_s: S) -> bool {
    (!FfaRxAcquireImplemented(old_s) ==> (result == NOT_SUPPORTED && new_s == old_s))
    && ((FfaRxAcquireImplemented(old_s)
            && !RxTxBufferPairRegistered(old_s, vm_id & 0xffffu32))
        ==> (result == INVALID_PARAMETERS && new_s == old_s))
    && ((FfaRxAcquireImplemented(old_s)
            && RxTxBufferPairRegistered(old_s, vm_id & 0xffffu32)
            && !CalleeCanRelinquishRxBuffer(old_s, vm_id & 0xffffu32))
        ==> (result == DENIED && new_s == old_s))
    && ((FfaRxAcquireImplemented(old_s)
            && RxTxBufferPairRegistered(old_s, vm_id & 0xffffu32)
            && CalleeCanRelinquishRxBuffer(old_s, vm_id & 0xffffu32))
        ==> (result == FFA_SUCCESS
            && RxBufferOwnedByCaller(new_s, vm_id & 0xffffu32)
            && RxTxBufferPairRegistered(new_s, vm_id & 0xffffu32)))
}
