pub open spec fn sdei_event_signal_spec(result: int64, old_s: S, new_s: S) -> bool {
    (result == SDEI_EVENT_SIGNAL_SUCCESS ==> (old_s.sdei_event_0_signaled == false && new_s.sdei_event_0_signaled == true))
    && (result == SDEI_EVENT_SIGNAL_NOT_SUPPORTED ==> (old_s.sdei_supported == false))
    && (result == SDEI_EVENT_SIGNAL_INVALID_PARAMETERS ==> (old_s.sdei_event_0_signaled == old_s.sdei_event_0_signaled))
    && (result != SDEI_EVENT_SIGNAL_SUCCESS && result != SDEI_EVENT_SIGNAL_NOT_SUPPORTED && result != SDEI_EVENT_SIGNAL_INVALID_PARAMETERS ==> (old_s.sdei_event_0_signaled == old_s.sdei_event_0_signaled))
}