pub open spec fn ffa_rx_acquire_spec(vm_id: UInt16, result: UInt32, error_code: Int32, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_RX_ACQUIRE) ==> ResultEqual(result, FFA_ERROR)) &&
  (ResultEqual(result, FFA_ERROR) ==> ResultEqual(error_code, NOT_SUPPORTED))
  && (!IsBufferPairRegistered(old_s, vm_id) ==> ResultEqual(result, FFA_ERROR)) &&
  (ResultEqual(result, FFA_ERROR) ==> ResultEqual(error_code, INVALID_PARAMETERS))
  && (!CalleeCanRelinquishRxBuffer(old_s, vm_id) ==> ResultEqual(result, FFA_ERROR)) &&
  (ResultEqual(result, FFA_ERROR) ==> ResultEqual(error_code, DENIED))
  && (ResultEqual(result, FFA_SUCCESS) ==> ResultEqual(result, FFA_SUCCESS))
  && (ResultEqual(result, FFA_SUCCESS) ==> RxBufferOwnedByCaller(new_s, vm_id))
  && ((IsImplementedAtInstance(old_s, FFA_RX_ACQUIRE) &&
       IsBufferPairRegistered(old_s, vm_id) &&
       CalleeCanRelinquishRxBuffer(old_s, vm_id))
    ==> ResultEqual(result, FFA_SUCCESS))
  && (result != FFA_SUCCESS
    ==> !RxBufferOwnedByCaller(new_s, vm_id))
}