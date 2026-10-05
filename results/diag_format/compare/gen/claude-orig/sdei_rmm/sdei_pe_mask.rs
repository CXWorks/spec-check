pub open spec fn sdei_pe_mask_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(old_s, CallingClient(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
    && (SdeiIsSupported(old_s, CallingClient(old_s)) ==> (
        PeIsMasked(new_s, CallingClient(old_s), CallingPe(old_s), NORMAL_PRIORITY)
        && PeIsMasked(new_s, CallingClient(old_s), CallingPe(old_s), CRITICAL_PRIORITY)
        && (!(PeIsMasked(old_s, CallingClient(old_s), CallingPe(old_s), NORMAL_PRIORITY)
              && PeIsMasked(old_s, CallingClient(old_s), CallingPe(old_s), CRITICAL_PRIORITY))
            ==> ResultEqual(result, 1))
        && ((PeIsMasked(old_s, CallingClient(old_s), CallingPe(old_s), NORMAL_PRIORITY)
             && PeIsMasked(old_s, CallingClient(old_s), CallingPe(old_s), CRITICAL_PRIORITY))
            ==> ResultEqual(result, 0))
    ))
}
