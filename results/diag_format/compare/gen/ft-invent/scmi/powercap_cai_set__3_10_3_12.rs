pub open spec fn powercap_cai_set__3_10_3_12_spec(domain_id: UInt32, flags: UInt32, cai: UInt32, cpli: UInt32, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result != RSI_SUCCESS ==> flags == 0)
  && (result == RSI_SUCCESS ==> cai != 0)
  && ((!(result == RSI_SUCCESS)) ==> flags == 0)
}