pub open spec fn sdei_event_unregister_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> (old_s.sdei_event_register_count == 0 || !old_s.sdei_event_registered(old_s.sdei_event_unregister_event)))
    && (result == RSI_ERROR_STATE ==> old_s.sdei_event_handler_running)
    && (result == RSI_SUCCESS ==> (new_s.sdei_event_registered(new_s.sdei_event_unregister_event) == false))
    && (result == RSI_INCOMPLETE ==> old_s.sdei_event_handler_running)
    && (result == RSI_ERROR_UNKNOWN ==> true)
}