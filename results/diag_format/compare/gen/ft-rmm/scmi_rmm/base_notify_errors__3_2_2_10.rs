pub open spec fn base_notify_errors__3_2_2_10_spec(notify_enable: UInt32, status: Int32, caller_agent: CallerAgent, old_s: S, new_s: S) -> bool {
  (!IsValidNotifyEnable(old_s, notify_enable) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) && (notify_enable[0] == 1) ==> ErrorNotifyEnabled(new_s, caller_agent))
  && (ResultEqual(status, SUCCESS) && (notify_enable[0] == 0) ==> !ErrorNotifyEnabled(new_s, caller_agent))
  && ((IsValidNotifyEnable(old_s, notify_enable))
    ==> ResultEqual(status, SUCCESS))
}