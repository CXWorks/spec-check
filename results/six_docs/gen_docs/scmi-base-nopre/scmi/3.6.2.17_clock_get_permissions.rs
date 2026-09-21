pub open spec fn 3.6.2.17_clock_get_permissions_spec(result: int32, permissions: uint32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> clock_id_is_invalid(old_s, clock_id))
    && (result == NOT_SUPPORTED ==> request_not_supported(old_s))
    && (result == SUCCESS ==> (permissions == old_s.clock_permissions(clock_id) && result == SUCCESS))
}