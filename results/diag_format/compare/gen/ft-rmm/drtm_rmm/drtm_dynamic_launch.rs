pub open spec fn drtm_dynamic_launch_spec(old_s: S, new_s: S) -> bool {
  DynamicLaunchRequestProxiedToCoprocessorDcrtm(new_s)
  && (CallerDmaProtectionsBlock(new_s, NONSECURE_DEVICES) && CallerDmaProtectionsBlock(new_s, SECURE_DEVICES))
  && (s: S, result: Result<(), RmiStatusCode> ==> true)
}