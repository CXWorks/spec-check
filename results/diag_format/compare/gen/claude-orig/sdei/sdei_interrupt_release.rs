pub open spec fn sdei_interrupt_release_spec(event: i32, result: i64, old_s: S, new_s: S) -> bool {
    (result == SUCCESS || result == NOT_SUPPORTED || result == INVALID_PARAMETERS || result == DENIED)
    && (result == NOT_SUPPORTED ==> new_s == old_s)
    && (result == INVALID_PARAMETERS ==> new_s == old_s)
    && (result == DENIED ==> new_s == old_s)
}
