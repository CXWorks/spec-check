pub open spec fn 3.5.6.16_performance_notify_limits_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (old_s.performance_notify_limits_domain_id == new_s.performance_notify_limits_domain_id))
    && (result == 0 ==> (old_s.performance_notify_limits_notify_enable == new_s.performance_notify_limits_notify_enable))
    && (result == 0 ==> (old_s.performance_notify_limits_reserved == new_s.performance_notify_limits_reserved))
    && (result == 1 ==> (old_s.performance_notify_limits_domain_id == new_s.performance_notify_limits_domain_id))
    && (result == 1 ==> (old_s.performance_notify_limits_notify_enable == new_s.performance_notify_limits_notify_enable))
    && (result == 1 ==> (old_s.performance_notify_limits_reserved == new_s.performance_notify_limits_reserved))
    && (result == 2 ==> (old_s.performance_notify_limits_domain_id == new_s.performance_notify_limits_domain_id))
    && (result == 2 ==> (old_s.performance_notify_limits_notify_enable == new_s.performance_notify_limits_notify_enable))
    && (result == 2 ==> (old_s.performance_notify_limits_reserved == new_s.performance_notify_limits_reserved))
    && (result == 3 ==> (old_s.performance_notify_limits_domain_id == new_s.performance_notify_limits_domain_id))
    && (result == 3 ==> (old_s.performance_notify_limits_notify_enable == new_s.performance_notify_limits_notify_enable))
    && (result == 3 ==> (old_s.performance_notify_limits_reserved == new_s.performance_notify_limits_reserved))
}