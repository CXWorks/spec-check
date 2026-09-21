pub open spec fn sdei_pe_unmask_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SDEI_SUCCESS ==> (new_s.sdei_pe_masked(old_s.pe_id) == false))
    && (result == SDEI_NOT_SUPPORTED ==> (old_s.sdei_supported() == false))
    && (result != SDEI_SUCCESS && result != SDEI_NOT_SUPPORTED ==> false)
}