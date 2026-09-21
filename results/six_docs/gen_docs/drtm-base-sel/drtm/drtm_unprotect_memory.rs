pub open spec fn drtm_unprotect_memory_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == DRTM_SUCCESS ==> (old_s.drtm_state == DRTM_PROTECTED && new_s.drtm_state == DRTM_UNPROTECTED))
    && (result == DRTM_DENIED ==> (old_s.drtm_state == DRTM_UNPROTECTED))
    && (result == DRTM_NOT_SUPPORTED ==> (old_s.drtm_state == DRTM_NOT_SUPPORTED))
    && (result == DRTM_INVALID_PARAMETERS ==> (old_s.drtm_state == DRTM_PROTECTED && new_s.drtm_state == DRTM_PROTECTED))
    && (result == DRTM_INTERNAL_ERROR ==> (old_s.drtm_state == DRTM_PROTECTED && new_s.drtm_state == DRTM_PROTECTED))
    && (result == DRTM_TPM_ERROR ==> (old_s.drtm_state == DRTM_PROTECTED && new_s.drtm_state == DRTM_PROTECTED))
    && (result == DRTM_OUT_OF_RESOURCE ==> (old_s.drtm_state == DRTM_PROTECTED && new_s.drtm_state == DRTM_PROTECTED))
    && (result == DRTM_INVALID_DATA ==> (old_s.drtm_state == DRTM_PROTECTED && new_s.drtm_state == DRTM_PROTECTED))
    && (result == DRTM_MEM_PROTECT_INVALID ==> (old_s.drtm_state == DRTM_PROTECTED && new_s.drtm_state == DRTM_PROTECTED))
    && (result == DRTM_NOT_FOUND ==> (old_s.drtm_state == DRTM_PROTECTED && new_s.drtm_state == DRTM_PROTECTED))
    && (result == DRTM_COPROCESSOR_ERROR ==> (old_s.drtm_state == DRTM_PROTECTED && new_s.drtm_state == DRTM_PROTECTED))
    && (result == DRTM_DENIED ==> (old_s.drtm_state == DRTM_UNPROTECTED))
}