pub open spec fn powercap_cap_notify__3_10_3_15_spec(domain_id: UInt32, notify_enable: UInt32, result: Result<(), RsiCommandReturnCode>, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
  && (!IsValidNotifyEnable(old_s, notify_enable) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result == RSI_SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == RSI_SUCCESS && (Bits(notify_enable, 0, 0) == 1) ==> CapChangeNotifyEnabled(new_s, caller, domain_id))
  && (result == RSI_SUCCESS && (Bits(notify_enable, 0, 0) == 0) ==> !CapChangeNotifyEnabled(new_s, caller, domain_id))
  && ((!(IsValidPowercapDomain(old_s, domain_id)) &&
       IsValidNotifyEnable(old_s, notify_enable))
    ==> ResultEqual(result, SUCCESS))
  && (result != RSI_SUCCESS
    ==> !CapChangeNotifyEnabled(new_s, caller, domain_id))
  && (result != RSI_SUCCESS
    ==> CapChangeNotifyEnabled(new_s, caller, domain_id))
}