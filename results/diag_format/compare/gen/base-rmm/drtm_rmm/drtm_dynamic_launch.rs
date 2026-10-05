pub open spec fn drtm_dynamic_launch_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (DynamicLaunchRequestProxiedToCoprocessorDcrtm() ==> result.is_Ok())
    && (CallerDmaProtectionsBlock(NONSECURE_DEVICES) && CallerDmaProtectionsBlock(SECURE_DEVICES) ==> result.is_Ok())
    && (result.is_Ok() ==> DynamicLaunchRequestProxiedToCoprocessorDcrtm())
    && (result.is_Ok() ==> CallerDmaProtectionsBlock(NONSECURE_DEVICES))
    && (result.is_Ok() ==> CallerDmaProtectionsBlock(SECURE_DEVICES))
}