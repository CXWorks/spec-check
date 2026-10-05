pub open spec fn sdei_pe_unmask_spec(fid: UInt32, client: UInt64, result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && (SdeiIsSupported(old_s) ==> (
        ResultEqual(result, SUCCESS)
        && !PeIsMasked(new_s, client, CallingPe(old_s))
        && PendingEventsDispatched(new_s, CallingPe(old_s))
    ))
}
