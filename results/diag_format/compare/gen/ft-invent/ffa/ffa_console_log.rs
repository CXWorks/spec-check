pub open spec fn ffa_console_log_spec(character_count: UInt32, character_lists: [UInt8; 16], result: Result<(), FfaStatusCode>, old_s: S, new_s: S) -> bool {
  ((character_count == 0) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))
  && ((character_count > 24) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))
  && ((character_count > 128) ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))
  && (result == FFA_SUCCESS ==> true)
  && (result == FFA_ERROR_NOT_SUPPORTED ==> true)
  && (result == FFA_ERROR_RETRY ==> true)
  && ((!(character_count == 0) &&
       !(character_count > 24) &&
       !(character_count > 128))
    ==> result == FFA_SUCCESS)
}