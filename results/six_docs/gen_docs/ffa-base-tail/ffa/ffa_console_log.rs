pub open spec fn ffa_console_log_spec(result: int, count: UInt32, old_s: S, new_s: S) -> bool {
    (count == 0 ==> ResultEqual(result, FFA_INVALID_PARAMETERS))
    && ((count as int) > 24 ==> ResultEqual(result, FFA_INVALID_PARAMETERS))
    && ((count as int) > 128 ==> ResultEqual(result, FFA_INVALID_PARAMETERS))
    && (result == FFA_SUCCESS ==> (count > 0))
    && (result == FFA_RETRY ==> (count > 0))
    && (result != FFA_SUCCESS && result != FFA_RETRY ==> count == 0)
    && (result == FFA_SUCCESS ==> old_s == new_s)
    && (result == FFA_RETRY ==> old_s == new_s)
}