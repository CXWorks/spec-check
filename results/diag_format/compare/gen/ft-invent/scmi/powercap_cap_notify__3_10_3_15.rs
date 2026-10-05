pub open spec fn powercap_cap_notify__3_10_3_15_spec(domain_id: uint32, notify_enable: uint32, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  ((notify_enable & 0xFFFFFFFE) != 0 ==> result == RSI_ERROR_INVALID_PARAMETERS)
  && (result == RSI_SUCCESS && (notify_enable & 0xFFFFFFFE) != 0 ==> result == RSI_ERROR_INVALID_PARAMETERS)
  && ((!( (notify_enable & 0xFFFFFFFE) != 0 ))
    ==> result == RSI_SUCCESS)
}