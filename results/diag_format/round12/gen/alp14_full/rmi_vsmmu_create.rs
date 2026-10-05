pub open spec fn rmi_vsmmu_create_spec(rd: Address, vsmmu_ptr: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da == FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && ((rd) % GRANULE_SIZE != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!is_physical_address_delegatable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleState(old_s, rd) != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((vsmmu_ptr) % GRANULE_SIZE != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!is_dram_physical_address_delegatable(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleState(old_s, vsmmu_ptr) != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((params_ptr) % GRANULE_SIZE != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!can_access_from_non_secure_physical_address_space(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!are_vsmmu_parameters_valid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!is_reg_base_or_reg_top_aligned_to_granule_size(old_s, VsmmuParametersAt(old_s, params_ptr).reg_base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!is_reg_base_or_reg_top_aligned_to_granule_size(old_s, VsmmuParametersAt(old_s, params_ptr).reg_top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!is_reg_base_or_reg_top_protected_address_in_realm(old_s, VsmmuParametersAt(old_s, params_ptr).reg_base, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!is_reg_base_or_reg_top_protected_address_in_realm(old_s, VsmmuParametersAt(old_s, params_ptr).reg_top, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VsmmuParametersAt(old_s, params_ptr).reg_top <= VsmmuParametersAt(old_s, params_ptr).reg_base ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> GranuleState(new_s, vsmmu_ptr) == VSMMU)
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).state == VSMMU_INACTIVE)
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).realm == rd)
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).reg_base == VsmmuParametersAt(new_s, params_ptr).reg_base)
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).reg_top == VsmmuParametersAt(new_s, params_ptr).reg_top)
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).aidr == VsmmuParametersAt(new_s, params_ptr).aidr)
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).idr[0] == VsmmuParametersAt(new_s, params_ptr).idr[0])
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).idr[1] == VsmmuParametersAt(new_s, params_ptr).idr[1])
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).idr[2] == VsmmuParametersAt(new_s, params_ptr).idr[2])
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).idr[3] == VsmmuParametersAt(new_s, params_ptr).idr[3])
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).idr[4] == VsmmuParametersAt(new_s, params_ptr).idr[4])
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).idr[5] == VsmmuParametersAt(new_s, params_ptr).idr[5])
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).idr[6] == VsmmuParametersAt(new_s, params_ptr).idr[6])
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_vsmmus == RealmAt(new_s, rd).num_vsmmus + 1)
  && ((!(ImplFeatures(old_s).feat_da == FEATURE_FALSE) &&
       ((rd) % GRANULE_SIZE == 0) &&
       (is_physical_address_delegatable(old_s, rd)) &&
       (GranuleState(old_s, rd) == RD) &&
       !(RealmAt(old_s, rd).state != REALM_NEW) &&
       ((vsmmu_ptr) % GRANULE_SIZE == 0) &&
       (is_dram_physical_address_delegatable(old_s, vsmmu_ptr)) &&
       (GranuleState(old_s, vsmmu_ptr) == DELEGATED) &&
       ((params_ptr) % GRANULE_SIZE == 0) &&
       (can_access_from_non_secure_physical_address_space(old_s, params_ptr)) &&
       (are_vsmmu_parameters_valid(old_s, params_ptr)) &&
       (is_reg_base_or_reg_top_aligned_to_granule_size(old_s, VsmmuParametersAt(old_s, params_ptr).reg_base)) &&
       (is_reg_base_or_reg_top_aligned_to_granule_size(old_s, VsmmuParametersAt(old_s, params_ptr).reg_top)) &&
       (is_reg_base_or_reg_top_protected_address_in_realm(old_s, VsmmuParametersAt(old_s, params_ptr).reg_base, rd)) &&
       (is_reg_base_or_reg_top_protected_address_in_realm(old_s, VsmmuParametersAt(old_s, params_ptr).reg_top, rd)) &&
       !(VsmmuParametersAt(old_s, params_ptr).reg_top <= VsmmuParametersAt(old_s, params_ptr).reg_base))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleState(new_s, vsmmu_ptr) == GranuleState(old_s, vsmmu_ptr))
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_vsmmus == RealmAt(old_s, rd).num_vsmmus)
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).state == VsmmuAt(old_s, vsmmu_ptr).state)
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).reg_base == VsmmuAt(old_s, vsmmu_ptr).reg_base)
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).reg_top == VsmmuAt(old_s, vsmmu_ptr).reg_top)
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).aidr == VsmmuAt(old_s, vsmmu_ptr).aidr)
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).idr[0] == VsmmuAt(old_s, vsmmu_ptr).idr[0])
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).idr[1] == VsmmuAt(old_s, vsmmu_ptr).idr[1])
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).idr[2] == VsmmuAt(old_s, vsmmu_ptr).idr[2])
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).idr[3] == VsmmuAt(old_s, vsmmu_ptr).idr[3])
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).idr[4] == VsmmuAt(old_s, vsmmu_ptr).idr[4])
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).idr[5] == VsmmuAt(old_s, vsmmu_ptr).idr[5])
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).idr[6] == VsmmuAt(old_s, vsmmu_ptr).idr[6])
}