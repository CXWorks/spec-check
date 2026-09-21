pub open spec fn 3.3.2.8_power_state_notify_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (old_s.power_state_notify_domain_id == domain_id && old_s.power_state_notify_enable == notify_enable))
    && (result == NOT_FOUND ==> (domain_id as int) < 0)
    && (result == INVALID_PARAMETERS ==> (notify_enable as int) & 0xFFFFFFFE != 0)
}