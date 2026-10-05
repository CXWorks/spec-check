pub open spec fn sdei_event_signal_spec(result: i64, event: i32, target_pe: u64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(old_s) ==> (result == NOT_SUPPORTED && new_s == old_s))
    && ((SdeiIsSupported(old_s) && event != 0) ==> (result == INVALID_PARAMETERS && new_s == old_s))
    && ((SdeiIsSupported(old_s) && event == 0 && !SdeiIsValidTargetPe(old_s, target_pe)) ==> (result == INVALID_PARAMETERS && new_s == old_s))
    && ((SdeiIsSupported(old_s) && event == 0 && SdeiIsValidTargetPe(old_s, target_pe)) ==> (result == SUCCESS && SdeiEventIsPending(new_s, target_pe, event as int)))
}
