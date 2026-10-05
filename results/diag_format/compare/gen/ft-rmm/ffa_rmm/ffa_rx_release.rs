pub open spec fn ffa_rx_release_spec(vm_id: UInt16, result: UInt32, error_code: Int32, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_RX_RELEASE, CurrentFfaInstance(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsRxTxPairRegisteredByHypervisor(old_s, vm_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!CallerOwnsRxBuffer(old_s, TargetRxBuffer(old_s, vm_id)) ==> ResultEqual(result, DENIED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> !CallerOwnsRxBuffer(new_s, TargetRxBuffer(new_s, vm_id)))
  && ((IsImplementedAtInstance(old_s, FFA_RX_RELEASE, CurrentFfaInstance(old_s)) &&
       IsRxTxPairRegisteredByHypervisor(old_s, vm_id) &&
       CallerOwnsRxBuffer(old_s, TargetRxBuffer(old_s, vm_id)))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> CallerOwnsRxBuffer(new_s, TargetRxBuffer(new_s, vm_id)))
}