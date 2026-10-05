pub open spec fn sdei_pe_mask_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (PeSdeiMasked(new_s, CallingClient(), CallingPe()) ==> result == 1)
    && (!PeSdeiMaskedBeforeCall(old_s, CallingClient(), CallingPe()) ==> result == 1)
    && (PeSdeiMaskedBeforeCall(old_s, CallingClient(), CallingPe()) ==> result == 0)
    && (!PeCanReceiveSdeiEvent(new_s, CallingClient(), CallingPe(), SDEI_PRIORITY_NORMAL))
    && (!PeCanReceiveSdeiEvent(new_s, CallingClient(), CallingPe(), SDEI_PRIORITY_CRITICAL))
    && (forall|e: SdeiEvent| SdeiEventStatus(e, new_s) == SdeiEventStatusBeforeCall(e, old_s))
    && (forall|p: Pe| p != CallingPe() ==> PeSdeiMasked(new_s, CallingClient(), p) == PeSdeiMaskedBeforeCall(old_s, CallingClient(), p))
    && (forall|c: Client| c != CallingClient() ==> PeSdeiMasked(c, new_s, CallingPe()) == PeSdeiMaskedBeforeCall(old_s, c, CallingPe()))
}