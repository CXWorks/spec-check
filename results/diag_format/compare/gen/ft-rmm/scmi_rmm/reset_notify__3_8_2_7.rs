pub open spec fn reset_notify__3_8_2_7_spec(domain_id: uint32, notify_enable: uint32, status: int32, old_s: S, new_s: S) -> bool {
  (!IsValidResetDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
  && (!IsValidNotifyEnable(old_s, notify_enable) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result == RSI_SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == RSI_SUCCESS ==> ResetNotifyEnabled(new_s, domain_id, calling_agent) == (notify_enable[0] == 1))
  && ((IsValidResetDomain(old_s, domain_id) &&
       IsValidNotifyEnable(old_s, notify_enable))
    ==> ResultEqual(result, SUCCESS))
  && (result != RSI_SUCCESS
    ==> ResetNotifyEnabled(new_s, domain_id, calling_agent) == ResetNotifyEnabled(old_s, domain_id, calling_agent))
}