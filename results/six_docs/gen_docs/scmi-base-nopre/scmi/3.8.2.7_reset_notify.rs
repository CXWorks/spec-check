pub open spec fn 3.8.2.7_reset_notify_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (old_s.domain_exists(old_s, domain_id) && (notify_enable == 0 || notify_enable == 1) && new_s.notify_enabled(old_s, domain_id) == notify_enable))
    && (result == -1 ==> !old_s.domain_exists(old_s, domain_id))
    && (result == -2 ==> (notify_enable < 0 || notify_enable > 1))
}