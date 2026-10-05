pub open spec fn ffa_console_log_spec(char_count: UInt8, char_list: [UInt32; 6], result: Result<FFAReturnCode, Int32>, logged_count: UInt32, old_s: S, new_s: S) -> bool {
  (CharCount(char_count) == 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsSmc32Convention(0) && CharCount(char_count) > 24 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsSmc64Convention(0) && CharCount(char_count) > 128 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsImplementedAtInstance(FFA_CONSOLE_LOG) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!AllCharactersLogged(char_list, char_count) ==> ResultEqual(result, RETRY))
  && (!AllCharactersLogged(char_list, char_count) ==> logged_count == NumCharactersLogged(char_list, char_count))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && ((!(CharCount(char_count) == 0) &&
       !(IsSmc32Convention(0) && CharCount(char_count) > 24) &&
       !(IsSmc64Convention(0) && CharCount(char_count) > 128) &&
       IsImplementedAtInstance(FFA_CONSOLE_LOG) &&
       AllCharactersLogged(char_list, char_count))
    ==> result == FFA_SUCCESS)
}