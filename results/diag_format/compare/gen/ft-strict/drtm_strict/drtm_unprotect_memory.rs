pub open spec fn drtm_unprotect_memory_spec(result: Result<(), NotSupported>, old_s: S, new_s: S) -> bool {
  (!IsDrtmSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!LaunchMemoryProtectionsInPlace(old_s) ==> ResultEqual(result, DENIED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> IsRegionBasedDmaProtection(new_s) ==> !LaunchMemoryProtectionsInPlace(new_s))
  && (result == SUCCESS ==> IsCompleteDmaProtection(new_s) ==> SmmuConfigurationUnchanged(new_s))
  && ((IsDrtmSupported(old_s) &&
       LaunchMemoryProtectionsInPlace(old_s))
    ==> result == SUCCESS)
}