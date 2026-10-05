pub open spec fn drtm_unprotect_memory_spec(old_s: S, new_s: S) -> bool {
  (!IsDrtmSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!MemoryProtectionsInPlace(old_s) ==> ResultEqual(result, DENIED))
  && (result == SUCCESS ==> !MemoryProtectionsInPlace(new_s))
  && (result == SUCCESS ==> (DmaProtectionType(new_s) == COMPLETE) ==> SmmuConfiguration(new_s) == SmmuConfiguration_pre(new_s))
  && (result == SUCCESS ==> (DmaProtectionType(new_s) == REGION_BASED) ==> !RegionProtectionsInPlace(new_s, DrtmParametersProtectedRegions(new_s)))
  && ((IsDrtmSupported(old_s) &&
       MemoryProtectionsInPlace(old_s))
    ==> ResultEqual(result, SUCCESS))
}