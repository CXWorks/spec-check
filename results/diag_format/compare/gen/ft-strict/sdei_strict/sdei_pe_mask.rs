pub open spec fn sdei_pe_mask_spec(result: Int64, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (PeSdeiMasked(new_s, CallingClient(), CallingPe()) ==> result == 1)
  && (!PeSdeiMaskedBeforeCall(old_s, CallingClient(), CallingPe()) ==> ResultEqual(result, 1))
  && (PeSdeiMaskedBeforeCall(old_s, CallingClient(), CallingPe()) ==> ResultEqual(result, 0))
  && (!PeCanReceiveSdeiEvent(new_s, CallingClient(), CallingPe(), SDEI_PRIORITY_NORMAL))
  && (!PeCanReceiveSdeiEvent(new_s, CallingClient(), CallingPe(), SDEI_PRIORITY_CRITICAL))
  && (forall (e: SdeiEvent), SdeiEventStatus(new_s, e) == SdeiEventStatusBeforeCall(new_s, e))
  && (forall (p: Pe), p != CallingPe() ==> PeSdeiMasked(new_s, CallingClient(), p) == PeSdeiMaskedBeforeCall(new_s, CallingClient(), p))
  && (forall (c: Client), c != CallingClient() ==> PeSdeiMasked(new_s, c, CallingPe()) == PeSdeiMaskedBeforeCall(new_s, c, CallingPe()))
  && ((SdeiIsSupported(old_s))
    ==> result == 1)
}