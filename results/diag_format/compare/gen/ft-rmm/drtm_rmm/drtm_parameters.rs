pub open spec fn drtm_parameters_spec(params: DRTM_PARAMETERS, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!IsNonSecure(old_s, params) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!IsPhysicallyContiguous(old_s, params) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!Is4KBAligned(old_s, params) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (AddressRangesOverlap(old_s, params) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (AnyAddressRangeWraps(old_s, params) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!DlmeImageWithinRegion(old_s, params.dlme_image_start, params.dlme_image_size, params.dlme_region_size) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (params.dlme_entry_offset >= params.dlme_image_size ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!DlmeRegionMeetsRequirements(old_s, params.dlme_region_addr, params.dlme_region_size) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (NwDceInUse(old_s, params) && (!IsNonSecure(old_s, params.nw_dce_region_addr, params.nw_dce_region_size) || !IsPhysicallyContiguous(old_s, params.nw_dce_region_addr, params.nw_dce_region_size)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (NwDceInUse(old_s, params) && !Is4KBAligned(old_s, params.nw_dce_region_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (params.launch_features[2..3] == 1 && params.launch_features[6] == 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!Is4KBAligned(old_s, params.mem_prot_table_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (params.revision != 2 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (params.reserved != 0 || params.launch_features[8..16] != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!NwDceInUse(old_s, params) && (params.nw_dce_region_addr != 0 || params.nw_dce_region_size != 0) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (params.launch_features[3..5] != 1 && (params.mem_prot_table_addr != 0 || params.mem_prot_table_size != 0) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((IsNonSecure(old_s, params) &&
       IsPhysicallyContiguous(old_s, params) &&
       Is4KBAligned(old_s, params) &&
       !AddressRangesOverlap(old_s, params) &&
       !AnyAddressRangeWraps(old_s, params) &&
       DlmeImageWithinRegion(old_s, params.dlme_image_start, params.dlme_image_size, params.dlme_region_size) &&
       !(params.dlme_entry_offset >= params.dlme_image_size) &&
       DlmeRegionMeetsRequirements(old_s, params.dlme_region_addr, params.dlme_region_size) &&
       !(NwDceInUse(old_s, params) && (!IsNonSecure(old_s, params.nw_dce_region_addr, params.nw_dce_region_size) || !IsPhysicallyContiguous(old_s, params.nw_dce_region_addr, params.nw_dce_region_size))) &&
       !(NwDceInUse(old_s, params) && !Is4KBAligned(old_s, params.nw_dce_region_addr)) &&
       !(params.launch_features[2..3] == 1 && params.launch_features[6] == 0) &&
       Is4KBAligned(old_s, params.mem_prot_table_addr) &&
       !(params.revision != 2) &&
       !(params.reserved != 0 || params.launch_features[8..16] != 0) &&
       NwDceInUse(old_s, params) &&
       !(params.launch_features[3..5] != 1 && (params.mem_prot_table_addr != 0 || params.mem_prot_table_size != 0)))
    ==> result.is_Ok())
}