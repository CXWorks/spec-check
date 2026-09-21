pub open spec fn sdei_private_reset_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SDEI_SUCCESS ==> (true))
    && (result == SDEI_NOT_SUPPORTED ==> (true))
    && (result == SDEI_DENIED ==> (true))
}