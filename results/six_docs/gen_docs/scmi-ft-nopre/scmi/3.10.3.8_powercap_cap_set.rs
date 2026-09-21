pub open spec fn 3.10.3.8_powercap_cap_set_spec(domain_id: UInt32, cpli: UInt32, flags: UInt32, power_cap: UInt32, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
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
    ==> PowerCapDomainAt(new_s, domain_id).power_cap == PowerCapDomainAt(old_s, domain_id).power_cap)
}