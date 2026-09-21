pub open spec fn 3.8.2.8_reset_domain_name_get_spec(domain_id: UInt32, result: RsiCommandReturnCode, flags: UInt32, name: [u8; 64], old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> flags == 0)
  && (result == RSI_SUCCESS ==> name[0] == 0)
  && ((!(result == RSI_SUCCESS))
    ==> flags == 0)
  && ((!(result == RSI_SUCCESS))
    ==> name[0] == 0)
}