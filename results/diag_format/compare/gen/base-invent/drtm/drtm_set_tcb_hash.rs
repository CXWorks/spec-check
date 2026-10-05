pub open spec fn drtm_set_tcb_hash_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> (old_s.drtm_tcb_hash_table_addr as int < 0 || old_s.drtm_tcb_hash_table_addr as int >= (1u64 << 48)))
    && (result == RSI_ERROR_STATE ==> old_s.drtm_tcb_hashes_locked)
    && (result == RSI_INCOMPLETE ==> true)
    && (result == RSI_ERROR_UNKNOWN ==> true)
    && (result == RSI_SUCCESS ==> (new_s.drtm_tcb_hashes_locked == old_s.drtm_tcb_hashes_locked) && (new_s.drtm_tcb_hash_table_addr == old_s.drtm_tcb_hash_table_addr))
}