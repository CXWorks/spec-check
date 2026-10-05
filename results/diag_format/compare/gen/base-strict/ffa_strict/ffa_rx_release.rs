pub open spec fn ffa_rx_release_spec(result: UInt32, old_s: S, new_s: S) -> bool {
    (!IsImplementedAtInstance(old_s, FFA_RX_RELEASE) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!HypervisorRegisteredBufferPairForVm(old_s, vm_id(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!HasRxBufferOwnership(old_s, caller(old_s), vm_id(old_s)) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, FFA_SUCCESS) ==> !HasRxBufferOwnership(new_s, caller(old_s), vm_id(old_s)))
    && (ResultEqual(result, FFA_SUCCESS) ==> RxBufferOwnership(new_s, caller(old_s), vm_id(old_s)) == RxBufferOwnership(old_s, caller(old_s), vm_id(old_s)))
}