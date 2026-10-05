pub open spec fn ffa_console_log_spec(result: Int32, logged_count: UInt32, old_s: S, new_s: S) -> bool {
    (CharCount(old_s.char_count) == 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (IsSmc32Convention(old_s.fid) && CharCount(old_s.char_count) > 24 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (IsSmc64Convention(old_s.fid) && CharCount(old_s.char_count) > 128 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsImplementedAtInstance(old_s, FFA_CONSOLE_LOG) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AllCharactersLogged(old_s.char_list, old_s.char_count) ==> (ResultEqual(result, RETRY) && logged_count == NumCharactersLogged(old_s.char_list, old_s.char_count)))
    && (ResultEqual(result, FFA_SUCCESS) ==> CharactersLoggedToConsole(old_s.char_list, old_s.char_count) within finite time)
}