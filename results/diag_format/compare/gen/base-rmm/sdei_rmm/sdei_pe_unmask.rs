pub open spec fn sdei_pe_unmask_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, SUCCESS) ==> (!PeIsMasked(old_s, client, CallingPe()) && !PeIsMasked(new_s, client, CallingPe()) && PendingEventsDispatched(CallingPe())))
}