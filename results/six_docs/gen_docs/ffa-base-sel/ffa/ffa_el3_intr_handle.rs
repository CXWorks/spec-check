pub open spec fn ffa_el3_intr_handle_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_NOT_SUPPORTED ==> (true))
    && (result == FFA_SUCCESS ==> (true))
}