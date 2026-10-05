pub open spec fn sdei_event_enable_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> (old_s.sdei_event_register(old_s.sdei_event_id) == false))
    && (result == RSI_ERROR_STATE ==> (old_s.sdei_event_handler_state(old_s.sdei_event_id) == SdeiEventHandlerStateHandlerUnregisterPending))
    && (result == RSI_SUCCESS ==> (new_s.sdei_event_enabled(new_s.sdei_event_id) == true))
}