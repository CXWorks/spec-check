pub open spec fn sdei_event_signal_spec(fid: UInt32, event: Int32, target_pe: UInt64, result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && ((SdeiIsSupported(old_s) && event != 0) ==> ResultEqual(result, INVALID_PARAMETERS))
    && ((SdeiIsSupported(old_s) && !IsValidMpidr(target_pe)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && ((SdeiIsSupported(old_s) && event == 0 && IsValidMpidr(target_pe)) ==> (
        ResultEqual(result, SUCCESS)
        && EventIsPending(new_s, event, target_pe)
    ))
}
