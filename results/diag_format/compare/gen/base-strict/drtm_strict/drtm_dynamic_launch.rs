pub open spec fn drtm_dynamic_launch_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (DynamicLaunchRequestProxiedToCoprocessorDcrtm(new_s) ==> result == RSI_SUCCESS)
    && (CallerDmaProtectionsBlockNonSecureDevices(new_s) ==> result == RSI_SUCCESS)
    && (CallerDmaProtectionsBlockSecureDevices(new_s) ==> result == RSI_SUCCESS)
}