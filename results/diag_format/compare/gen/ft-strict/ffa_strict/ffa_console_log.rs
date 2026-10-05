pub open spec fn ffa_console_log_spec(char_count: UInt32, characters: [UInt32; 6], result: Result<FFAStatus, Int32>, logged_count: UInt32, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_CONSOLE_LOG) ==> ResultEqual(result, NOT_SUPPORTED))
  && (char_count == 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsSmc32Convention(old_s, 0x8400008A) && char_count > 24 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsSmc64Convention(old_s, 0x8400008A) && char_count > 128 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!AllCharactersLogged(old_s, characters, char_count as int) ==> ResultEqual(result, RETRY(0)))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> CharactersLoggedToConsoleInFiniteTime(new_s, characters, char_count as int))
  && ((IsImplementedAtInstance(old_s, FFA_CONSOLE_LOG) &&
       char_count != 0 &&
       !(IsSmc32Convention(old_s, 0x8400008A) && char_count > 24) &&
       !(IsSmc64Convention(old_s, 0x8400008A) && char_count > 128) &&
       AllCharactersLogged(old_s, characters, char_count as int))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> CharactersLoggedToConsoleInFiniteTime(new_s, characters, char_count as int))
  && (result != FFA_SUCCESS
    ==> logged_count == 0)
  && (result != RETRY(0)
    ==> logged_count == 0)
  && (result == RETRY(0)
    ==> logged_count > 0)
  && (result != FFA_SUCCESS && result != RETRY(0)
    ==> logged_count == 0)
}