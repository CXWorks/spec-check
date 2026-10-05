pub open spec fn powercap_cap_notify__3_10_3_15_spec(domain_id: uint32, notify_enable: uint32, status: int32, old_s: S, new_s: S) -> bool {
  (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
  && (!IsValidNotifyEnable(old_s, notify_enable) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> PowercapCapNotifyEnabled(new_s, domain_id, calling_agent) == notify_enable[0])
  && ((IsValidPowercapDomain(old_s, domain_id) &&
       IsValidNotifyEnable(old_s, notify_enable))
    ==> ResultEqual(result, SUCCESS))
  && (result != SUCCESS
    ==> PowercapCapNotifyEnabled(new_s, domain_id, calling_agent) == PowercapCapNotifyEnabled(old_s, domain_id, calling_agent))
}