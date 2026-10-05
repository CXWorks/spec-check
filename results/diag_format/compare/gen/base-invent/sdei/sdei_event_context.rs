pub open spec fn sdei_event_context_spec(result: int64, old_s: S, new_s: S) -> bool {
    let param_id: UInt32 = old_s.sdei_event_context_param_id;
    let handler_running: bool = old_s.sdei_event_context_handler_running;
    let calling_pe: UInt32 = old_s.sdei_event_context_calling_pe;
    let event_handler_pe: UInt32 = old_s.sdei_event_context_event_handler_pe;
    let sdei_supported: bool = old_s.sdei_event_context_sdei_supported;
    let param_id_valid: bool = param_id <= 17;
    let handler_running_true: bool = handler_running;
    let same_pe: bool = calling_pe == event_handler_pe;
    let success: bool = result.is_Ok();
    (!sdei_supported ==> ResultEqual(result, SDEI_ERROR_NOT_SUPPORTED))
    && (!param_id_valid ==> ResultEqual(result, SDEI_ERROR_INVALID_PARAMETERS))
    && (!handler_running_true ==> ResultEqual(result, SDEI_ERROR_DENIED))
    && (!same_pe ==> ResultEqual(result, SDEI_ERROR_DENIED))
    && (handler_running_true && param_id_valid && same_pe && sdei_supported ==> (result.is_Ok() && new_s == old_s))
}