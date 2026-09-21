pub open spec fn drtm_set_tcb_hash_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == DRTM_NOT_SUPPORTED ==> true)
    && (result == DRTM_INVALID_PARAMETERS ==> true)
    && (result == DRTM_INVALID_DATA ==> true)
    && (result == DRTM_OUT_OF_RESOURCE ==> true)
    && (result == DRTM_DENIED ==> true)
    && (result == DRTM_SUCCESS ==> true)
}