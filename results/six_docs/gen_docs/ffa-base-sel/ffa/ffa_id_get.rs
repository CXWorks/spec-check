pub open spec fn ffa_id_get_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_ABORTED ==> false)
    && (result == FFA_BUSY ==> false)
    && (result == FFA_DENIED ==> false)
    && (result == FFA_INTERRUPTED ==> false)
    && (result == FFA_INVALID_PARAMETERS ==> false)
    && (result == FFA_NOT_READY ==> false)
    && (result == FFA_NOT_SUPPORTED ==> (result == FFA_NOT_SUPPORTED))
    && (result == FFA_NO_DATA ==> false)
    && (result == FFA_NO_MEMORY ==> false)
    && (result == FFA_RETRY ==> false)
    && (result == FFA_SUCCESS ==> (result >= 0 && result <= 0xFFFF))
}