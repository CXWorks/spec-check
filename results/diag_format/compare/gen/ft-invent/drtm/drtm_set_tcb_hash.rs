pub open spec fn drtm_set_tcb_hash_spec_tcb_hash_table: X1, result: DrtmSetTcbHashReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> DRTM_FEATURES(new_s).max_tcb_hash_table_entries > 0)
  && (result == RSI_SUCCESS ==> DRTM_FEATURES(new_s).max_tcb_hash_table_entries <= DRTM_FEATURES(old_s).max_tcb_hash_table_entries)
  && ((!(DRTM_FEATURES(old_s).drtm_supported) &&
       !(DRTM_FEATURES(old_s).drtm_supported))
    ==> RSI_ERROR_NOT_SUPPORTED)
  && (result == RSI_INVALID_PARAMETERS
    ==> DRTM_FEATURES(new_s).max_tcb_hash_table_entries == DRTM_FEATURES(old_s).max_tcb_hash_table_entries)
  && (result == RSI_INVALID_DATA
    ==> DRTM_FEATURES(new_s).max_tcb_hash_table_entries == DRTM_FEATURES(old_s).max_tcb_hash_table_entries)
  && (result == RSI_OUT_OF_RESOURCE
    ==> DRTM_FEATURES(new_s).max_tcb_hash_table_entries == DRTM_FEATURES(old_s).max_tcb_hash_table_entries)
  && (result == RSI_DENIED
    ==> DRTM_FEATURES(new_s).max_tcb_hash_table_entries == DRTM_FEATURES(old_s).max_tcb_hash_table_entries)
  && ((!(DRTM_FEATURES(old_s).drtm_supported) &&
       !(DRTM_FEATURES(old_s).drtm_supported))
    ==> result == RSI_ERROR_NOT_SUPPORTED)
  && ((!(DRTM_FEATURES(old_s).drtm_supported) &&
       !(DRTM_FEATURES(old_s).drtm_supported))
    ==> result == RSI_ERROR_NOT_SUPPORTED)
  && (result != RSI_SUCCESS
    ==> DRTM_FEATURES(new_s).max_tcb_hash_table_entries == DRTM_FEATURES(old_s).max_tcb_hash_table_entries)
}