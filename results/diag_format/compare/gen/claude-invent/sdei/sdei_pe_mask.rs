pub open spec fn sdei_pe_mask_spec(result: i64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(old_s) ==> (result == NOT_SUPPORTED && new_s == old_s))
    && ((SdeiIsSupported(old_s) && !SdeiPeIsMasked(old_s)) ==> (result == 1 && SdeiPeIsMasked(new_s) && SdeiPeMaskOnlyChanged(old_s, new_s)))
    && ((SdeiIsSupported(old_s) && SdeiPeIsMasked(old_s)) ==> (result == 0 && SdeiPeIsMasked(new_s) && SdeiPeMaskOnlyChanged(old_s, new_s)))
}
