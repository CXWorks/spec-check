pub open spec fn drtm_unprotect_memory_spec(result: i64, old_s: S, new_s: S) -> bool {
    (!DrtmIsSupported(old_s) ==> result == NOT_SUPPORTED)
    && ((DrtmIsSupported(old_s) && !DrtmMemoryProtectionInPlace(old_s)) ==> (result == DENIED))
    && ((DrtmIsSupported(old_s) && DrtmMemoryProtectionInPlace(old_s) && DrtmDmaProtectionIsComplete(old_s)) ==> (
        result == SUCCESS
        && SmmuConfigEqual(old_s, new_s)
        && !DrtmMemoryProtectionInPlace(new_s)
    ))
    && ((DrtmIsSupported(old_s) && DrtmMemoryProtectionInPlace(old_s) && DrtmDmaProtectionIsRegionBased(old_s)) ==> (
        result == SUCCESS
        && DrtmRegionProtectionsRemoved(old_s, new_s)
        && !DrtmMemoryProtectionInPlace(new_s)
    ))
}
