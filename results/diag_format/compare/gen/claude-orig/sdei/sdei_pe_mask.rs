pub open spec fn sdei_pe_mask_spec(result: i64, old_s: S, new_s: S) -> bool {
    (result == 1 || result == 0 || result == NOT_SUPPORTED)
}
