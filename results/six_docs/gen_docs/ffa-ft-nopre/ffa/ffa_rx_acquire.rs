pub open spec fn ffa_rx_acquire_spec(vm_id: UInt32, result: FfaReturn, old_s: S, new_s: S) -> bool {
  (result == FFA_SUCCESS ==> true)
  && (result == FFA_ERROR(DENIED) ==> true)
  && (result == FFA_ERROR(INVALID_PARAMETERS) ==> true)
  && (result == FFA_ERROR(NOT_SUPPORTED) ==> true)
  && ((!(result == FFA_SUCCESS) &&
       !(result == FFA_ERROR(DENIED)) &&
       !(result == FFA_ERROR(INVALID_PARAMETERS)) &&
       !(result == FFA_ERROR(NOT_SUPPORTED)))
    ==> true)
}