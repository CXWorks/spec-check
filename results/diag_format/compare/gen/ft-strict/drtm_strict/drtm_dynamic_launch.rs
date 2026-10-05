pub open spec fn drtm_dynamic_launch_spec(old_s: S, new_s: S) -> bool {
  (DynamicLaunchRequestProxiedToCoprocessorDcrtm(new_s))
  && (CallerDmaProtectionsBlockNonSecureDevices(new_s) && CallerDmaProtectionsBlockSecureDevices(new_s))
  && (DynamicLaunchRequestProxiedToCoprocessorDcrtm(old_s) ==> DynamicLaunchRequestProxiedToCoprocessorDcrtm(new_s))
  && (CallerDmaProtectionsBlockNonSecureDevices(old_s) && CallerDmaProtectionsBlockSecureDevices(old_s) ==> (CallerDmaProtectionsBlockNonSecureDevices(new_s) && CallerDmaProtectionsBlockSecureDevices(new_s)))
  && ((!(DynamicLaunchRequestProxiedToCoprocessorDcrtm(new_s)) ||
       !(CallerDmaProtectionsBlockNonSecureDevices(new_s) && CallerDmaProtectionsBlockSecureDevices(new_s)))
    ==> (DynamicLaunchRequestProxiedToCoprocessorDcrtm(old_s) &&
         CallerDmaProtectionsBlockNonSecureDevices(old_s) &&
         CallerDmaProtectionsBlockSecureDevices(old_s)))
}