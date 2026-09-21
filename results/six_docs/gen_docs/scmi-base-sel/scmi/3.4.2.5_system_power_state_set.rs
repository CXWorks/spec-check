pub open spec fn 3.4.2.5_system_power_state_set_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> (old_s.flags & 0x1 == 0 || (old_s.system_state as int) < 0 || (old_s.system_state as int) > 0x7FFFFFFF && (old_s.system_state as int) < 0x80000000))
    && (result == SCMI_NOT_SUPPORTED ==> true)
    && (result == SCMI_DENIED ==> (old_s.system_state as int) == 0x4 && (exists ap: Agent, ap != old_s.caller && ap.is_running(old_s) || ap.is_idle(old_s)))
    && (result == SCMI_SUCCESS ==> (new_s.system_state as int) == old_s.system_state)
    && (result == SCMI_SUCCESS ==> (new_s.flags & 0x1 == 0))
    && (result == SCMI_SUCCESS ==> (new_s.flags & 0xFFFFFFFE == old_s.flags & 0xFFFFFFFE))
    && (result != SCMI_SUCCESS && result != SCMI_INVALID_PARAMETERS && result != SCMI_NOT_SUPPORTED && result != SCMI_DENIED ==> true)
}