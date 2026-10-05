pub open spec fn system_power_state_notify__3_4_2_7_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!SystemPowerStateNotifySupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && (Bits64(old_s.cmd_input_notify_enable, 31, 1) != 0 || !IsPermissibleNotifyEnable(old_s, old_s.cmd_input_notify_enable) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, SUCCESS) ==> (Bits64(old_s.cmd_input_notify_enable, 0, 0) == 1 ==> SystemPowerStateNotifyEnabled(new_s) && Bits64(old_s.cmd_input_notify_enable, 0, 0) == 0 ==> !SystemPowerStateNotifyEnabled(new_s)))
    && (SystemPowerStateNotifyEnabled(new_s) ==> Bits64(old_s.cmd_input_notify_enable, 0, 0) == 1)
    && (!SystemPowerStateNotifyEnabled(new_s) ==> Bits64(old_s.cmd_input_notify_enable, 0, 0) == 0)
}