pub open spec fn drtm_set_tcb_hash_spec_tcb_hash_table: X1, result: DrtmSetTcbHashReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> TcbHashTableValid(new_s, tcb_hash_table))
  && (result == RSI_ERROR_NOT_SUPPORTED ==> TcbHashTableUnchanged(new_s, tcb_hash_table))
  && (result == RSI_ERROR_INVALID_PARAMETERS ==> TcbHashTableUnchanged(new_s, tcb_hash_table))
  && (result == RSI_ERROR_INVALID_DATA ==> TcbHashTableUnchanged(new_s, tcb_hash_table))
  && (result == RSI_ERROR_OUT_OF_RESOURCE ==> TcbHashTableUnchanged(new_s, tcb_hash_table))
  && (result == RSI_ERROR_DENIED ==> TcbHashTableUnchanged(new_s, tcb_hash_table))
  && ((!(TcbHashTableValid(old_s, tcb_hash_table)) &&
       result != RSI_SUCCESS)
    ==> TcbHashTableUnchanged(new_s, tcb_hash_table))
}