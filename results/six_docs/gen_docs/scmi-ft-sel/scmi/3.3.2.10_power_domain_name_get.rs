pub open spec fn 3.3.2.10_power_domain_name_get_spec(domain_id: UInt32, flags: UInt32, ext_name: [UInt8; 64], result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> flags == 0)
}