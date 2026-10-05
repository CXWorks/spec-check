pub open spec fn drtm_unprotect_memory_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_NOT_SUPPORTED ==> !DrtmSupported(old_s))
    && (result == RSI_ERROR_DENIED ==> (DrtmSupported(old_s) && !HasMemoryProtections(old_s)))
    && (result == RSI_SUCCESS ==> (DrtmSupported(old_s) && HasMemoryProtections(old_s) && !HasMemoryProtections(new_s)))
    && (result != RSI_SUCCESS && result != RSI_ERROR_NOT_SUPPORTED && result != RSI_ERROR_DENIED ==> true)
}