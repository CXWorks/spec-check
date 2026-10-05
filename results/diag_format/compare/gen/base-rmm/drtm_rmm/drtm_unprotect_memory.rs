pub open spec fn drtm_unprotect_memory_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!IsDrtmSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!MemoryProtectionsInPlace(old_s) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> !MemoryProtectionsInPlace(new_s))
    && (DmaProtectionType() == COMPLETE ==> SmmuConfiguration(new_s) == SmmuConfiguration(old_s))
    && (DmaProtectionType() == REGION_BASED ==> !RegionProtectionsInPlace(new_s, DrtmParametersProtectedRegions()))
}