pub open spec fn sdei_event_context_spec(result: int64, old_s: S, new_s: S) -> bool {
    (old_s.sdei_handler_running(old_s.pe_id) == false ==> ResultEqual(result, SDEI_DENIED))
    && (old_s.sdei_param_id_invalid(old_s.pe_id, old_s.sdei_param_id) ==> ResultEqual(result, SDEI_INVALID_PARAMETERS))
    && (old_s.sdei_handler_running(old_s.pe_id) == true && old_s.sdei_param_id_valid(old_s.pe_id, old_s.sdei_param_id) ==> result == old_s.sdei_register_value(old_s.pe_id, old_s.sdei_param_id))
}