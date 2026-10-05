pub open spec fn drtm_parameters_spec(params: DrtmParameters, old_s: S, new_s: S) -> bool {
  (params.revision != 2 ==> DynamicLaunchFails(new_s, params))
  && (params.reserved != 0 ==> DynamicLaunchFails(new_s, params))
  && (Bits(params.launch_features, 31, 8) != 0 ==> DynamicLaunchFails(new_s, params))
  && (!IsNonSecureContiguous(old_s, ParamsAddr(params), DrtmParametersSize(params)) ==> DynamicLaunchFails(new_s, params))
  && (!IsAligned(old_s, ParamsAddr(params), 4096) ==> DynamicLaunchFails(new_s, params))
  && (ParameterRangesOverlap(old_s, params) ==> DynamicLaunchFails(new_s, params))
  && (ParameterRangesWrap(old_s, params) ==> DynamicLaunchFails(new_s, params))
  && (params.dlme_image_start + params.dlme_image_size > params.dlme_region_size ==> DynamicLaunchFails(new_s, params))
  && (params.dlme_entry_point_offset >= params.dlme_image_size ==> DynamicLaunchFails(new_s, params))
  && (!DlmeRegionIsValid(old_s, params.dlme_region_address, params.dlme_region_size) ==> DynamicLaunchFails(new_s, params))
  && (NwdDceInUse(old_s, params) && !IsNonSecureContiguous(old_s, params.nwd_dce_region_address, params.nwd_dce_region_size) ==> DynamicLaunchFails(new_s, params))
  && (NwdDceInUse(old_s, params) && !IsAligned(old_s, params.nwd_dce_region_address, 4096) ==> DynamicLaunchFails(new_s, params))
  && (!NwdDceInUse(old_s, params) && (params.nwd_dce_region_address != 0 || params.nwd_dce_region_size != 0) ==> DynamicLaunchFails(new_s, params))
  && (Bits(params.launch_features, 2, 1) == 1 && Bits(params.launch_features, 6, 6) == 0 ==> DynamicLaunchFails(new_s, params))
  && (!IsAligned(old_s, params.mpt_address, 4096) ==> DynamicLaunchFails(new_s, params))
  && (Bits(params.launch_features, 5, 3) != 1 && (params.mpt_address != 0 || params.mpt_size != 0) ==> DynamicLaunchFails(new_s, params))
  && ((!(params.revision != 2) &&
       !(params.reserved != 0) &&
       !(Bits(params.launch_features, 31, 8) != 0) &&
       IsNonSecureContiguous(old_s, ParamsAddr(params), DrtmParametersSize(params)) &&
       IsAligned(old_s, ParamsAddr(params), 4096) &&
       !(ParameterRangesOverlap(old_s, params)) &&
       !(ParameterRangesWrap(old_s, params)) &&
       !(params.dlme_image_start + params.dlme_image_size > params.dlme_region_size) &&
       !(params.dlme_entry_point_offset >= params.dlme_image_size) &&
       DlmeRegionIsValid(old_s, params.dlme_region_address, params.dlme_region_size) &&
       !(NwdDceInUse(old_s, params) && !IsNonSecureContiguous(old_s, params.nwd_dce_region_address, params.nwd_dce_region_size)) &&
       !(NwdDceInUse(old_s, params) && !IsAligned(old_s, params.nwd_dce_region_address, 4096)) &&
       (NwdDceInUse(old_s, params) && !((params.nwd_dce_region_address != 0) || (params.nwd_dce_region_size != 0))) &&
       !((Bits(params.launch_features, 2, 1) == 1) && (Bits(params.launch_features, 6, 6) == 0)) &&
       IsAligned(old_s, params.mpt_address, 4096) &&
       !((Bits(params.launch_features, 5, 3) != 1) && ((params.mpt_address != 0) || (params.mpt_size != 0))))
    ==> true)
}