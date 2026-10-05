pub open spec fn voltage_domain_name_get__3_9_2_11_spec(domain_id: UInt32, result: RsiCommandReturnCode, flags: UInt32, name: [UInt8; 64], old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> flags == 0)
  && (result == RSI_SUCCESS ==> name[0] == 0)
  && ((!(result == RSI_SUCCESS))
    ==> flags == 0)
  && ((!(result == RSI_SUCCESS))
    ==> name[0] == 0)
}