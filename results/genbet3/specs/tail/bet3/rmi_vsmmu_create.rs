pub open spec fn rmi_vsmmu_create_spec(rd: Address, vsmmu_ptr: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm().static.feat_vsmmu != FEATURE_TRUE ==> result == RMI_ERROR_NOT_SUPPORTED)
  && (!AddrIsRmiGranuleAligned(old_s, rd) ==> result == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rd) ==> result == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result == RMI_ERROR_INPUT)
  && (RealmAt(old_s, rd).state != REALM_NEW ==> result == RMI_ERROR_REALM(0))
  && (!AddrIsRmiGranuleAligned(old_s, vsmmu_ptr) ==> result == RMI_ERROR_INPUT)
  && (!PaIsPopulatedConventional(old_s, vsmmu_ptr) ==> result == RMI_ERROR_INPUT)
  && (!PaIsTrackedFine(old_s, vsmmu_ptr) ==> (result == RMI_ERROR_TRACKING(0, TrackingToRmiResult(old_s, vsmmu_ptr)) && result.get_Err_0().level_addr.addr == AddrShiftRmiGranule(old_s, vsmmu_ptr))))
  && (GranuleAt(old_s, vsmmu_ptr).state != GRAN_DELEGATED ==> result == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, params_ptr) ==> result == RMI_ERROR_INPUT)
  && (!NonSecureAccessPermitted(old_s, params_ptr) ==> result == RMI_ERROR_INPUT)
  && (!RmiVsmmuParamsIsValid(old_s, params_ptr) ==> result == RMI_ERROR_INPUT)
  && (!RmiVsmmuRegsAreCompatible(old_s, RmiVsmmuParamsAt(old_s, params_ptr).aidr,RmiVsmmuParamsAt(old_s, params_ptr).idr,RmiVsmmuParamsAt(old_s, params_ptr).iiidr) ==> result == RMI_ERROR_INPUT)
  && ((!AddrIsRmiGranuleAligned(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_base) || !AddrIsRmiGranuleAligned(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_top)) ==> result == RMI_ERROR_INPUT)
  && ((!AddrIsProtected(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_base, RealmAt(old_s, rd)) || !AddrIsProtected(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_top, RealmAt(old_s, rd)) || (RmiVsmmuParamsAt(old_s, params_ptr).reg_top) <= (RmiVsmmuParamsAt(old_s, params_ptr).reg_base)) ==> result == RMI_ERROR_INPUT)
  && (result.is_Ok() ==> GranuleAt(new_s, vsmmu_ptr).state == GRAN_VSMMU)
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).state == VSMMU_INACTIVE)
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).realm == rd)
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).reg_base == RmiVsmmuParamsAt(new_s, params_ptr).reg_base)
  && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).reg_top == RmiVsmmuParamsAt(new_s, params_ptr).reg_top)
  && (result.is_Ok() ==> RmiVsmmuRegsAreCompatible(new_s, VsmmuAt(new_s, vsmmu_ptr).aidr,VsmmuAt(new_s, vsmmu_ptr).idr,VsmmuAt(new_s, vsmmu_ptr).iiidr))
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_vsmmus == RealmAt(new_s, rd).num_vsmmus + 1)
  && ((!(Rmm().static.feat_vsmmu != FEATURE_TRUE) &&
       AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       !(RealmAt(old_s, rd).state != REALM_NEW) &&
       AddrIsRmiGranuleAligned(old_s, vsmmu_ptr) &&
       PaIsPopulatedConventional(old_s, vsmmu_ptr) &&
       PaIsTrackedFine(old_s, vsmmu_ptr) &&
       !(GranuleAt(old_s, vsmmu_ptr).state != GRAN_DELEGATED) &&
       AddrIsRmiGranuleAligned(old_s, params_ptr) &&
       NonSecureAccessPermitted(old_s, params_ptr) &&
       RmiVsmmuParamsIsValid(old_s, params_ptr) &&
       RmiVsmmuRegsAreCompatible(old_s, RmiVsmmuParamsAt(old_s, params_ptr).aidr,RmiVsmmuParamsAt(old_s, params_ptr).idr,RmiVsmmuParamsAt(old_s, params_ptr).iiidr) &&
       (AddrIsRmiGranuleAligned(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_base) && AddrIsRmiGranuleAligned(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_top)) &&
       (AddrIsProtected(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_base, RealmAt(old_s, rd)) && AddrIsProtected(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_top, RealmAt(old_s, rd)) && !((RmiVsmmuParamsAt(old_s, params_ptr).reg_top) <= (RmiVsmmuParamsAt(old_s, params_ptr).reg_base))))
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
    ==> RealmAt(new_s, rd).num_vsmmus == RealmAt(old_s, rd).num_vsmmus)
  && (!(result.is_Ok() && (RmiVsmmuParamsAt(old_s, params_ptr).reg_top) <= (RmiVsmmuParamsAt(old_s, params_ptr).reg_base)) ==> result == RMI_ERROR_INPUT)
}