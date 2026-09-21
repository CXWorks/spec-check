pub open spec fn drtm_unprotect_memory_spec(result: int64, old_s: S, new_s: S) -> bool {
    (result == DRTM_SUCCESS ==> true)
    && (result == DRTM_DENIED ==> true)
    && (result == DRTM_NOT_SUPPORTED ==> true)
    && (result == DRTM_INVALID_PARAMETERS ==> true)
    && (result == DRTM_INVALID_DATA ==> true)
    && (result == DRTM_MEM_PROTECT_INVALID ==> true)
    && (result == DRTM_OUT_OF_RESOURCE ==> true)
    && (result == DRTM_TPM_ERROR ==> true)
    && (result == DRTM_COPROCESSOR_ERROR ==> true)
    && (result == DRTM_INTERNAL_ERROR ==> true)
    && (result == DRTM_NOT_FOUND ==> true)
}