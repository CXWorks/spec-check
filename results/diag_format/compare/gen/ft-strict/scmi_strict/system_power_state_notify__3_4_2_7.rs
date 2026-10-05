pub open spec fn system_power_state_notify__3_4_2_7_spec(notify_enable: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!SystemPowerStateNotifySupported(old_s, caller) ==> ResultEqual(status, NOT_SUPPORTED))
  && ((Bits(notify_enable, 31, 1) != 0 || !IsPermissibleNotifyEnable(old_s, caller, notify_enable)) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) && Bits(notify_enable, 0, 0) == 1 ==> SystemPowerStateNotifyEnabled(new_s, caller))
  && (ResultEqual(status, SUCCESS) && Bits(notify_enable, 0, 0) == 0 ==> !SystemPowerStateNotifyEnabled(new_s, caller))
  && ((!(SystemPowerStateNotifySupported(old_s, caller)) &&
       !((Bits(notify_enable, 31, 1) != 0 || !IsPermissibleNotifyEnable(old_s, caller, notify_enable))))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> !SystemPowerStateNotifyEnabled(new_s, caller))
  && (result != SUCCESS
    ==> SystemPowerStateNotifyEnabled(new_s, caller))
}