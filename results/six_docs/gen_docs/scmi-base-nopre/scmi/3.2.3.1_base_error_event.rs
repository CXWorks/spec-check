pub open spec fn base_error_event_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!old_s.base_error_event_enabled ==> result.is_Ok())
    && (result.is_Ok() ==> old_s.base_error_event_enabled == new_s.base_error_event_enabled)
}