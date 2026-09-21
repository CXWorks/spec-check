pub open spec fn sdei_pe_mask_spec(result: int64, old_s: S, new_s: S) -> bool {
    (result == SDEI_SUCCESS ==> (new_s.pe_masked(old_s.pe_id()) == true))
    && (result == SDEI_DENIED ==> (new_s.pe_masked(old_s.pe_id()) == true))
    && (result == SDEI_INVALID_PARAMETERS ==> (new_s.pe_masked(old_s.pe_id()) == false))
    && (result == SDEI_NOT_SUPPORTED ==> (new_s.pe_masked(old_s.pe_id()) == false))
    && (result == SDEI_OUT_OF_RESOURCE ==> (new_s.pe_masked(old_s.pe_id()) == false))
    && (result == SDEI_PENDING ==> (new_s.pe_masked(old_s.pe_id()) == false))
    && (result == SDEI_SUCCESS || result == SDEI_DENIED || result == SDEI_INVALID_PARAMETERS || result == SDEI_NOT_SUPPORTED || result == SDEI_OUT_OF_RESOURCE || result == SDEI_PENDING)
}