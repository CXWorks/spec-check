pub open spec fn ffa_notification_bitmap_create_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> (old_s == new_s))
    && (result == FFA_NOT_SUPPORTED ==> (old_s == new_s))
    && (result == FFA_DENIED ==> (old_s == new_s))
    && (result == FFA_NO_MEMORY ==> (old_s == new_s))
    && (result == FFA_ABORTED ==> (old_s == new_s))
    && (result == FFA_BUSY ==> (old_s == new_s))
    && (result == FFA_INTERRUPTED ==> (old_s == new_s))
    && (result == FFA_RETRY ==> (old_s == new_s))
    && (result == FFA_NO_DATA ==> (old_s == new_s))
    && (result == FFA_NOT_READY ==> (old_s == new_s))
    && (result == FFA_DENIED ==> (old_s == new_s))
    && (result == FFA_NO_MEMORY ==> (old_s == new_s))
}