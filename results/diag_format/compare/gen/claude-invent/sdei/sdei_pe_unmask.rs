pub open spec fn sdei_pe_unmask_spec(result: i64, old_s: S, new_s: S) -> bool {
    (!SdeiIsImplemented(old_s) ==> (result == NOT_SUPPORTED && new_s == old_s))
    && (SdeiIsImplemented(old_s) ==> (
        result == SUCCESS
        && !SdeiCallingPeIsMasked(new_s)
        && SdeiOtherPesMaskUnchanged(old_s, new_s)
        && SdeiPendingEventsDispatchedForCallingPe(old_s, new_s)
    ))
}
