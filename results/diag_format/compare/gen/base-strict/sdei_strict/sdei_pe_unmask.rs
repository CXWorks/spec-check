pub open spec fn sdei_pe_unmask_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiImplementedForClient(old_s, CallingClient()) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, SUCCESS) ==> (!PeMaskedForPriority(new_s, CallingClient(), CallingPe(), NORMAL) && !PeMaskedForPriority(new_s, CallingClient(), CallingPe(), CRITICAL) && forall|pe: Pe| pe != CallingPe() ==> PeMaskStateUnchanged(new_s, CallingClient(), pe) && forall|ev: Event| EventStatusUnchanged(ev) && PendingEventsDispatched(new_s, CallingClient(), CallingPe())))
}