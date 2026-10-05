pub open spec fn powercap_measurements_get__3_10_3_14_spec(domain_id: UInt32, result: RsiCommandReturnCode, power: UInt32, mai: UInt32, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> power == 0)
  && (result == RSI_SUCCESS ==> mai == 0)
  && ((!(result == RSI_SUCCESS))
    ==> power == power)
  && ((!(result == RSI_SUCCESS))
    ==> mai == mai)
}