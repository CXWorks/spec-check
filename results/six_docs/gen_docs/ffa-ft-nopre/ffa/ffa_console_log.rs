pub open spec fn ffa_console_log_spec(character_count: UInt32, character_lists: [UInt8; 16], result: Result<(), FfaStatusCode>, old_s: S, new_s: S) -> bool {
  (result == FFA_ERROR(INVALID_PARAMETERS) && (character_count == 0))
  && (result == FFA_ERROR(INVALID_PARAMETERS) && (character_count > 24))
  && (result == FFA_ERROR(NOT_SUPPORTED))
  && (result == FFA_ERROR(RETRY))
  && ((!(result == FFA_ERROR(INVALID_PARAMETERS) && (character_count == 0)) &&
       !(result == FFA_ERROR(INVALID_PARAMETERS) && (character_count > 24)) &&
       !(result == FFA_ERROR(NOT_SUPPORTED)) &&
       !(result == FFA_ERROR(RETRY)))
    ==> result == FFA_SUCCESS)
  && (result == FFA_ERROR(RETRY)
    ==> character_count <= 128)
  && ((!(result == FFA_ERROR(INVALID_PARAMETERS) && (character_count == 0)) &&
       !(result == FFA_ERROR(INVALID_PARAMETERS) && (character_count > 24)) &&
       !(result == FFA_ERROR(NOT_SUPPORTED)) &&
       result == FFA_SUCCESS)
    ==> character_count <= 128)
}