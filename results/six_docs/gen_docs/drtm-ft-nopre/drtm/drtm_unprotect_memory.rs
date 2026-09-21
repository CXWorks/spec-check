pub open spec fn drtm_unprotect_memory_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS && (DrtmParameters(old_s).protection_type == PROTECTION_TYPE_REGION))
  ==> (SmmuConfig(new_s) == SmmuConfig(old_s))
  && (result != RSI_SUCCESS
    ==> (SmmuConfig(new_s) == SmmuConfig(old_s)))
}