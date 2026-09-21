pub open spec fn ffa_version_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_ABORTED || result == FFA_BUSY || result == FFA_DENIED || result == FFA_INTERRUPTED || result == FFA_INVALID_PARAMETERS || result == FFA_NOT_READY || result == FFA_NOT_SUPPORTED || result == FFA_NO_DATA || result == FFA_NO_MEMORY || result == FFA_RETRY) ==> true
    && (result == FFA_INVALID_PARAMETERS ==> (result == FFA_INVALID_PARAMETERS))
    && (result == FFA_NOT_SUPPORTED ==> (result == FFA_NOT_SUPPORTED))
    && (result == FFA_INVALID_PARAMETERS ==> (result == FFA_INVALID_PARAMETERS))
    && (result == FFA_NOT_SUPPORTED ==> (result == FFA_NOT_SUPPORTED))
    && true
}