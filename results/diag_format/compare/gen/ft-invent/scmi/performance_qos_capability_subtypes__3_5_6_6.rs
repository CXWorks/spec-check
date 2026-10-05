pub open spec fn performance_qos_capability_subtypes__3_5_6_6_spec(domain_id: uint32, capability_type: uint32, capability_subtypes: uint32, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> capability_subtypes == 0)
  && (result == RSI_SUCCESS ==> (capability_type & 0xFFFF00FF) == 0)
  && (result == RSI_SUCCESS ==> (capability_type & 0x0000FF00) == 0)
  && (result == RSI_SUCCESS ==> (capability_type & 0x000000FF) > 0 && (capability_type & 0x000000FF) == 1)
  && ((!(result == RSI_SUCCESS) &&
       !(result == RSI_ERROR_INPUT) &&
       !(result == RSI_ERROR_STATE) &&
       !(result == RSI_INCOMPLETE) &&
       !(result == RSI_ERROR_UNKNOWN))
    ==> capability_subtypes == 0)
}