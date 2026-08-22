pub open spec fn rmi_vsmmu_create_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S, rd: Address, vsmmu_ptr: Address, params_ptr: Address) -> bool {
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
    && ((!AddrIsGranuleAligned(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_base)
            || !AddrIsGranuleAligned(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_top))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!AddrIsProtected(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_base, RealmAt(old_s, rd))
            || !AddrIsProtected(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_top, RealmAt(old_s, rd))
            || RmiVsmmuParamsAt(old_s, params_ptr).reg_top <= RmiVsmmuParamsAt(old_s, params_ptr).reg_base)
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && RealmAt(old_s, rd).state == REALM_NEW
        && AddrIsGranuleAligned(old_s, vsmmu_ptr)
        && PaIsDelegableDram(old_s, vsmmu_ptr)
        && GranuleAt(old_s, vsmmu_ptr).state == DELEGATED
        && AddrIsGranuleAligned(old_s, params_ptr)
        && GranuleAccessPermitted(old_s, params_ptr, PAS_NS)
        && RmiVsmmuParamsIsValid(old_s, params_ptr)
        && AddrIsGranuleAligned(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_base)
        && AddrIsGranuleAligned(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_top)
        && AddrIsProtected(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_base, RealmAt(old_s, rd))
        && AddrIsProtected(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_top, RealmAt(old_s, rd))
        && RmiVsmmuParamsAt(old_s, params_ptr).reg_top > RmiVsmmuParamsAt(old_s, params_ptr).reg_base)
        ==> (result.is_Ok()
            && GranuleAt(new_s, vsmmu_ptr).state == VSMMU
            && VsmmuAt(new_s, vsmmu_ptr).state == VSMMU_INACTIVE
            && VsmmuAt(new_s, vsmmu_ptr).realm == rd
            && VsmmuAt(new_s, vsmmu_ptr).reg_base == RmiVsmmuParamsAt(old_s, params_ptr).reg_base
            && VsmmuAt(new_s, vsmmu_ptr).reg_top == RmiVsmmuParamsAt(old_s, params_ptr).reg_top
            && VsmmuAt(new_s, vsmmu_ptr).aidr == RmiVsmmuParamsAt(old_s, params_ptr).aidr
            && VsmmuAt(new_s, vsmmu_ptr).idr[0] == RmiVsmmuParamsAt(old_s, params_ptr).idr[0]
            && VsmmuAt(new_s, vsmmu_ptr).idr[1] == RmiVsmmuParamsAt(old_s, params_ptr).idr[1]
            && VsmmuAt(new_s, vsmmu_ptr).idr[2] == RmiVsmmuParamsAt(old_s, params_ptr).idr[2]
            && VsmmuAt(new_s, vsmmu_ptr).idr[3] == RmiVsmmuParamsAt(old_s, params_ptr).idr[3]
            && VsmmuAt(new_s, vsmmu_ptr).idr[4] == RmiVsmmuParamsAt(old_s, params_ptr).idr[4]
            && VsmmuAt(new_s, vsmmu_ptr).idr[5] == RmiVsmmuParamsAt(old_s, params_ptr).idr[5]
            && VsmmuAt(new_s, vsmmu_ptr).idr[6] == RmiVsmmuParamsAt(old_s, params_ptr).idr[6]
            && RealmAt(new_s, rd).num_vsmmus == RealmAt(old_s, rd).num_vsmmus + 1))
}