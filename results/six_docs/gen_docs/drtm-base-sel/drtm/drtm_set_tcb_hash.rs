pub open spec fn drtm_set_tcb_hash_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == DRTM_SUCCESS ==> (new_s.drtm_tcb_hash_table == old_s.drtm_tcb_hash_table))
    && (result == DRTM_NOT_SUPPORTED ==> (new_s.drtm_tcb_hash_table == old_s.drtm_tcb_hash_table))
    && (result == DRTM_INVALID_PARAMETERS ==> (new_s.drtm_tcb_hash_table == old_s.drtm_tcb_hash_table))
    && (result == DRTM_INVALID_DATA ==> (new_s.drtm_tcb_hash_table == old_s.drtm_tcb_hash_table))
    && (result == DRTM_OUT_OF_RESOURCE ==> (new_s.drtm_tcb_hash_table == old_s.drtm_tcb_hash_table))
    && (result == DRTM_DENIED ==> (new_s.drtm_tcb_hash_table == old_s.drtm_tcb_hash_table))
    && (result == DRTM_SUCCESS ==> (new_s.drtm_tcb_hash_table == old_s.drtm_tcb_hash_table))
}