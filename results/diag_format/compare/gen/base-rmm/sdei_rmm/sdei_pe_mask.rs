pub open spec fn sdei_pe_mask_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(CallingClient()) ==> ResultEqual(result, NOT_SUPPORTED))
    && (PeIsMasked(CallingClient(), CallingPe(), NORMAL_PRIORITY) && PeIsMasked(CallingClient(), CallingPe(), CRITICAL_PRIORITY))
    && (!old(PeIsMasked(CallingClient(), CallingPe())) ==> ResultEqual(result, 1))
    && (old(PeIsMasked(CallingClient(), CallingPe())) ==> ResultEqual(result, 0))
}