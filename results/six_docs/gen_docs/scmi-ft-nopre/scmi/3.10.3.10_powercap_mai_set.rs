pub open spec fn 3.10.3.10_powercap_mai_set_spec(domain_id: UInt32, flags: UInt32, mai: UInt32, result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS)
  && (result == RSI_NOT_FOUND)
  && (result == RSI_NOT_SUPPORTED)
  && (result == RSI_INVALID_PARAMETERS)
  && (result == RSI_DENIED)
  && ((!(result == RSI_SUCCESS) &&
       !(result == RSI_NOT_FOUND) &&
       !(result == RSI_NOT_SUPPORTED) &&
       !(result == RSI_INVALID_PARAMETERS) &&
       !(result == RSI_DENIED))
    ==> PowerCapDomainAt(new_s, domain_id).mai == PowerCapDomainAt(old_s, domain_id).mai)
}