pub open spec fn sdei_pe_mask_spec(result: Result<(), SdeiStatusCode>, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s, CallingClient()) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result.is_Ok() ==> PeIsMasked(new_s, CallingClient(), CallingPe(), NORMAL_PRIORITY))
  && (result.is_Ok() ==> PeIsMasked(new_s, CallingClient(), CallingPe(), CRITICAL_PRIORITY))
  && (result.is_Ok() ==> !PeIsMasked(old_s, CallingClient(), CallingPe()) ==> ResultEqual(result, 1))
  && (result.is_Ok() ==> PeIsMasked(old_s, CallingClient(), CallingPe()) ==> ResultEqual(result, 0))
  && ((SdeiIsSupported(old_s, CallingClient()))
    ==> result.is_Ok())
}