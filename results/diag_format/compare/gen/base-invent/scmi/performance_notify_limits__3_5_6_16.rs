pub open spec fn performance_notify_limits__3_5_6_16_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> !DomainExists(old_s, domain_id))
    && (result == NOT_SUPPORTED ==> !DomainSupportsNotifications(old_s, domain_id))
    && (result == INVALID_PARAMETERS ==> notify_enable != 0 && notify_enable != 1)
    && (result == SUCCESS ==> DomainExists(old_s, domain_id) && DomainSupportsNotifications(old_s, domain_id) && (notify_enable == 0 || notify_enable == 1))
}