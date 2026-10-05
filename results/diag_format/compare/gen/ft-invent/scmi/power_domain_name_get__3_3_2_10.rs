pub open spec fn power_domain_name_get__3_3_2_10_spec(domain_id: UInt32, result: RsiCommandReturnCode, flags: UInt32, ext_name: [UInt8; 64], old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> flags == 0)
  && (result == RSI_SUCCESS ==> ext_name[0] == 0)
  && ((!(result == RSI_SUCCESS))
    ==> flags == 0)
  && ((!(result == RSI_SUCCESS))
    ==> ext_name[0] == 0)
}