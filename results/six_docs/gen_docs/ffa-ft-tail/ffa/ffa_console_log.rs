pub open spec fn ffa_console_log_spec(character_count: UInt32, character_lists: [UInt8; 16], result: int32, successfully_logged: UInt32, old_s: S, new_s: S) -> bool {
  (result == FFA_INVALID_PARAMETERS && (character_count == 0 || character_count > 24 || character_count > 128))
  && (result == FFA_NOT_SUPPORTED)
  && (result == FFA_RETRY ==> successfully_logged > 0)
  && ((!(result == FFA_INVALID_PARAMETERS && (character_count == 0 || character_count > 24 || character_count > 128)) &&
       result == FFA_SUCCESS)
    ==> true)
  && (result != FFA_SUCCESS && result != FFA_RETRY
    ==> successfully_logged == 0)
}