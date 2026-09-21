pub open spec fn sdei_event_routing_set_spec(result: int64, old_s: S, new_s: S) -> bool {
    (result == SDEI_INVALID_PARAMETERS ==> (old_s.cmd_input_event != 0 || !old_s.is_shared_event(old_s.cmd_input_event) || (old_s.cmd_input_routing_mode != 0 && old_s.cmd_input_routing_mode != 1) || (old_s.cmd_input_routing_mode == 1 && !old_s.is_valid_affinity(old_s.cmd_input_affinity))))
    && (result == SDEI_DENIED ==> old_s.cmd_input_event_handler_state != SDEI_EVENT_HANDLER_STATE_REGISTERED)
    && (result == SDEI_NOT_SUPPORTED ==> true)
    && (result == SDEI_SUCCESS ==> old_s.cmd_input_event_handler_state == SDEI_EVENT_HANDLER_STATE_REGISTERED && old_s.cmd_input_event != 0 && old_s.is_shared_event(old_s.cmd_input_event) && (old_s.cmd_input_routing_mode == 0 || old_s.cmd_input_routing_mode == 1) && (old_s.cmd_input_routing_mode == 0 || old_s.is_valid_affinity(old_s.cmd_input_affinity)))
}