pub open spec fn sdei_pe_mask_spec(result: int64, old_s: S, new_s: S) -> bool {
    (result == 1 ==> <true>)
    && (result == 0 ==> <true>)
    && (result == SDEI_NOT_SUPPORTED ==> <true>)
}