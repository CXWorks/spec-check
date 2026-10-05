pub open spec fn drtm_unprotect_memory_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!IsDrtmSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!LaunchMemoryProtectionsInPlace(old_s) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (IsRegionBasedDmaProtection(old_s) ==> !LaunchMemoryProtectionsInPlace(new_s)))
    && (ResultEqual(result, SUCCESS) ==> (IsCompleteDmaProtection(old_s) ==> SmmuConfigurationUnchanged(old_s, new_s)))
}