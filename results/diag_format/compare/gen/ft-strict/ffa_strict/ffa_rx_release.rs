pub open spec fn ffa_rx_release_spec(vm_id: UInt16, result: Result<UInt32, Int32>, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_RX_RELEASE, ffa_instance) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!HypervisorRegisteredBufferPairForVm(old_s, vm_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!HasRxBufferOwnership(old_s, caller, vm_id) ==> ResultEqual(result, DENIED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> !HasRxBufferOwnership(new_s, caller, vm_id))
  && ((IsImplementedAtInstance(old_s, FFA_RX_RELEASE, ffa_instance) &&
       HypervisorRegisteredBufferPairForVm(old_s, vm_id) &&
       HasRxBufferOwnership(old_s, caller, vm_id))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> HasRxBufferOwnership(new_s, caller, vm_id))
}