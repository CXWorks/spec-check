pub open spec fn powercap_mai_set__3_10_3_10_spec(domain_id: UInt32, flags: UInt32, mai: UInt32, result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> flags == 0)
  && (result == RSI_SUCCESS ==> mai != 0)
  && ((!(result == RSI_SUCCESS))
    ==> flags == 0)
  && ((!(result == RSI_SUCCESS))
    ==> mai != 0)
}