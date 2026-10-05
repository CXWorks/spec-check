pub open spec fn sdei_event_signal_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (event != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsValidMpidr(target_pe) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, SUCCESS) ==> EventIsPending(event, target_pe))
    && (ResultEqual(result, SUCCESS) ==> EventPendingState(event, target_pe) == EventPendingState(old_s, target_pe))
}