pub open spec fn rmi_vsmmu_create_spec(rd: Address, vsmmu_ptr: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegableDram(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vsmmu_ptr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RmiVsmmuParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!AddrIsGranuleAligned(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_base) || !AddrIsGranuleAligned(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_top)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!AddrIsProtected(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_base, RealmAt(old_s, rd)) || !AddrIsProtected(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_top, RealmAt(old_s, rd)) || (RmiVsmmuParamsAt(old_s, params_ptr).reg_top) <= (RmiVsmmuParamsAt(old_s, params_ptr).reg_base)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> GranuleAt(new_s, vsmmu_ptr).state == VSMMU)
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).state == VSMMU_INACTIVE)
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).realm == rd)
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).reg_base == RmiVsmmuParamsAt(new_s, params_ptr).reg_base)
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).reg_top == RmiVsmmuParamsAt(new_s, params_ptr).reg_top)
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).aidr == RmiVsmmuParamsAt(new_s, params_ptr).aidr)
  && (result.is_Ok() ==> (VsmmuAt(new_s, vsmmu_ptr).idr[0] == RmiVsmmuParamsAt(new_s, params_ptr).idr[0] && VsmmuAt(new_s, vsmmu_ptr).idr[1] == RmiVsmmuParamsAt(new_s, params_ptr).idr[1] && VsmmuAt(new_s, vsmmu_ptr).idr[2] == RmiVsmmuParamsAt(new_s, params_ptr).idr[2] && VsmmuAt(new_s, vsmmu_ptr).idr[3] == RmiVsmmuParamsAt(new_s, params_ptr).idr[3] && VsmmuAt(new_s, vsmmu_ptr).idr[4] == RmiVsmmuParamsAt(new_s, params_ptr).idr[4] && VsmmuAt(new_s, vsmmu_ptr).idr[5] == RmiVsmmuParamsAt(new_s, params_ptr).idr[5] && VsmmuAt(new_s, vsmmu_ptr).idr[6] == RmiVsmmuParamsAt(new_s, params_ptr).idr[6]))
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_vsmmus == RealmAt(new_s, rd).num_vsmmus + 1)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !(RealmAt(old_s, rd).state != REALM_NEW) &&
       AddrIsGranuleAligned(old_s, vsmmu_ptr) &&
       PaIsDelegableDram(old_s, vsmmu_ptr) &&
       !(GranuleAt(old_s, vsmmu_ptr).state != DELEGATED) &&
       AddrIsGranuleAligned(old_s, params_ptr) &&
       GranuleAccessPermitted(old_s, params_ptr, PAS_NS) &&
       RmiVsmmuParamsIsValid(old_s, params_ptr) &&
       !((!AddrIsGranuleAligned(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_base) || !AddrIsGranuleAligned(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_top))) &&
       !((!AddrIsProtected(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_base, RealmAt(old_s, rd)) || !AddrIsProtected(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_top, RealmAt(old_s, rd)) || (RmiVsmmuParamsAt(old_s, params_ptr).reg_top) <= (RmiVsmmuParamsAt(old_s, params_ptr).reg_base))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, vsmmu_ptr).state == GranuleAt(old_s, vsmmu_ptr).state)
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).state == VsmmuAt(old_s, vsmmu_ptr).state)
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).realm == VsmmuAt(old_s, vsmmu_ptr).realm)
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).reg_base == VsmmuAt(old_s, vsmmu_ptr).reg_base)
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).reg_top == VsmmuAt(old_s, vsmmu_ptr).reg_top)
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).aidr == VsmmuAt(old_s, vsmmu_ptr).aidr)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_vsmmus == RealmAt(old_s, rd).num_vsmmus)
}