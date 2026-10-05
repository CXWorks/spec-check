pub open spec fn ffa_rx_acquire_spec(vm_id: UInt16, result: UInt32, error_code: Int32, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_RX_ACQUIRE) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsBufferPairRegistered(old_s, vm_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!CanRelinquishRxBufferOwnership(old_s, vm_id) ==> ResultEqual(result, DENIED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> RxBufferOwner(new_s, vm_id) == Caller)
  && ((IsImplementedAtInstance(old_s, FFA_RX_ACQUIRE) &&
       IsBufferPairRegistered(old_s, vm_id) &&
       CanRelinquishRxBufferOwnership(old_s, vm_id))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> RxBufferOwner(new_s, vm_id) == RxBufferOwner(old_s, vm_id))
}