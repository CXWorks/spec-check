pub open spec fn sdei_pe_unmask_spec(result: Result<(), SdeiStatusCode>, old_s: S, new_s: S) -> bool {
  (!SdeiImplementedForClient(old_s, CallingClient()) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> !PeMaskedForPriority(new_s, CallingClient(), CallingPe(), NORMAL))
  && (result == SUCCESS ==> !PeMaskedForPriority(new_s, CallingClient(), CallingPe(), CRITICAL))
  && (result == SUCCESS ==> (forall (pe: Pe), pe != CallingPe() ==> PeMaskStateUnchanged(new_s, CallingClient(), pe)))
  && (result == SUCCESS ==> (forall (ev: Event), EventStatusUnchanged(new_s, ev)))
  && (result == SUCCESS ==> PendingEventsDispatched(new_s, CallingClient(), CallingPe()))
  && ((SdeiImplementedForClient(old_s, CallingClient()))
    ==> result == SUCCESS)
}