pub open spec fn sdei_event_context_spec(result: int64, old_s: S, new_s: S) -> bool {
    (old_s.handler_running == false ==> result == SDEI_DENIED)
    && (old_s.param_id > 17 ==> result == SDEI_INVALID_PARAMETERS)
    && (old_s.handler_running == true && old_s.param_id <= 17 ==> result == old_s.reg[old_s.param_id as int])
}