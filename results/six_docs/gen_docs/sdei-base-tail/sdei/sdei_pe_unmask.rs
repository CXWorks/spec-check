pub open spec fn sdei_pe_unmask_spec(result: int64, old_s: S, new_s: S) -> bool {
    (result == SDEI_SUCCESS ==> true)
    && (result == SDEI_NOT_SUPPORTED ==> true)
}