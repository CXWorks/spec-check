pub open spec fn drtm_enable_secure_interrupts_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS)
}