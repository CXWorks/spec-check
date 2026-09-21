pub open spec fn 3.10.3.7_powercap_cap_get_spec(domain_id: UInt32, cpli: UInt32, result: RsiCommandReturnCode, power_cap: UInt32, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> power_cap == 0 || power_cap > 0)
}