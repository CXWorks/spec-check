pub open spec fn sdei_pe_unmask_spec(result: Result<(), SdeiStatusCode>, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> !PeIsMasked(new_s, client(new_s), CallingPe(new_s)))
  && (result == SUCCESS ==> PendingEventsDispatched(new_s, CallingPe(new_s)))
  && ((SdeiIsSupported(old_s))
    ==> result == SUCCESS)
}