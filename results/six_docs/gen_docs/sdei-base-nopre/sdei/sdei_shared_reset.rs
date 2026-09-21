pub open spec fn sdei_shared_reset_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (RsiCommandReturnCode::RSI_ERROR_INPUT == result ==> (false))
    && (RsiCommandReturnCode::RSI_SUCCESS == result ==> (
        !old_s.sdei_shared_events_running()
        && !old_s.sdei_interrupt_bindings_registered()
    ))
    && (RsiCommandReturnCode::RSI_ERROR_STATE == result ==> (
        old_s.sdei_shared_events_running() || old_s.sdei_interrupt_bindings_registered()
    ))
    && (RsiCommandReturnCode::RSI_INCOMPLETE == result ==> (
        false
    ))
    && (RsiCommandReturnCode::RSI_ERROR_UNKNOWN == result ==> (
        false
    ))
    && (RsiCommandReturnCode::RSI_NOT_SUPPORTED == result ==> (
        !old_s.sdei_supported()
    ))
    && (RsiCommandReturnCode::RSI_DENIED == result ==> (
        old_s.sdei_shared_events_running() || old_s.sdei_interrupt_bindings_registered()
    ))
}