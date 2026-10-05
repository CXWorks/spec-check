pub open spec fn ffa_msg_send2_spec(sender_vm_id: UInt32, flags: UInt32, result: Result<(), FfaStatusCode>, old_s: S, new_s: S) -> bool {
  (result == FFA_ERROR(INVALID_PARAMETERS) ==> sender_vm_id != 0)
  && (result == FFA_ERROR(INVALID_PARAMETERS) ==> result.unwrap_err() == INVALID_PARAMETERS)
  && (result == FFA_ERROR(BUSY) ==> result.unwrap_err() == BUSY)
  && (result == FFA_ERROR(DENIED) ==> result.unwrap_err() == DENIED)
  && (result == FFA_ERROR(NO_MEMORY) ==> result.unwrap_err() == NO_MEMORY)
  && (result == FFA_ERROR(NOT_SUPPORTED) ==> result.unwrap_err() == NOT_SUPPORTED)
  && ((!(result == FFA_ERROR(INVALID_PARAMETERS)) &&
       !(result == FFA_ERROR(BUSY)) &&
       !(result == FFA_ERROR(DENIED)) &&
       !(result == FFA_ERROR(NO_MEMORY)) &&
       !(result == FFA_ERROR(NOT_SUPPORTED)))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> result == FFA_ERROR(INVALID_PARAMETERS))
  && (result != FFA_SUCCESS
    ==> result == FFA_ERROR(BUSY))
  && (result != FFA_SUCCESS
    ==> result == FFA_ERROR(DENIED))
  && (result != FFA_SUCCESS
    ==> result == FFA_ERROR(NO_MEMORY))
  && (result != FFA_SUCCESS
    ==> result == FFA_ERROR(NOT_SUPPORTED))
}