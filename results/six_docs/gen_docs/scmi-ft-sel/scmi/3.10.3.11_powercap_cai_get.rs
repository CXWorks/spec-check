pub open spec fn 3.10.3.11_powercap_cai_get_spec(domain_id: UInt32, cpli: UInt32, result: RsiCommandReturnCode, cai: UInt32, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> cai == 0)
}