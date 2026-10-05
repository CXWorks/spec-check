pub open spec fn sdei_event_disable_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> (old_s.sdei_event_is_registered(old_s.sdei_event_id) == false))
    && (result == RSI_ERROR_STATE ==> (old_s.sdei_event_handler_state(old_s.sdei_event_id) == SdeiEventHandlerState::HandlerUnregisterPending))
    && (result == RSI_SUCCESS ==> (new_s.sdei_event_is_registered(new_s.sdei_event_id) == old_s.sdei_event_is_registered(old_s.sdei_event_id))
        && (new_s.sdei_event_handler_state(new_s.sdei_event_id) == SdeiEventHandlerState::Disabled))
}