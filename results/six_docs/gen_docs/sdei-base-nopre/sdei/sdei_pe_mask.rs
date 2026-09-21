pub open spec fn sdei_pe_mask_spec(result: int64, old_s: S, new_s: S) -> bool {
    (result == 1 ==> (new_s.sdei_pe_masked(old_s.pe_id) == true))
    && (result == 0 ==> (new_s.sdei_pe_masked(old_s.pe_id) == true))
    && (result == NOT_SUPPORTED ==> (new_s.sdei_pe_masked(old_s.pe_id) == old_s.sdei_pe_masked(old_s.pe_id)))
    && (result != 1 && result != 0 && result != NOT_SUPPORTED ==> false)
}