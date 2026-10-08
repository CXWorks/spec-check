pub open spec fn rmi_vsmmu_create_spec(rd: Address, vsmmu_ptr: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && (
            !AddrIsGranuleAligned(old_s, rd)
            || !PaIsDelegable(old_s, rd)
            || GranuleAt(old_s, rd).state != RD
            || (AddrIsGranuleAligned(old_s, rd)
                && PaIsDelegable(old_s, rd)
                && GranuleAt(old_s, rd).state == RD
                && RealmAt(old_s, rd).state != REALM_NEW)
            || !AddrIsGranuleAligned(old_s, vsmmu_ptr)
            || !PaIsDelegableDram(old_s, vsmmu_ptr)
            || GranuleAt(old_s, vsmmu_ptr).state != DELEGATED
            || !AddrIsGranuleAligned(old_s, params_ptr)
            || !GranuleAccessPermitted(old_s, params_ptr, PAS_NS)
            || !RmiVsmmuParamsIsValid(old_s, params_ptr)
            || (AddrIsGranuleAligned(old_s, params_ptr)
                && GranuleAccessPermitted(old_s, params_ptr, PAS_NS)
                && RmiVsmmuParamsIsValid(old_s, params_ptr)
                && (
                    !AddrIsGranuleAligned(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_base)
                    || !AddrIsGranuleAligned(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_top)
                    || !AddrIsProtected(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_base, RealmAt(old_s, rd))
                    || !AddrIsProtected(old_s, RmiVsmmuParamsAt(old_s, params_ptr).reg_top, RealmAt(old_s, rd))
                    || (RmiVsmmuParamsAt(old_s, params_ptr).reg_top as int) <= (RmiVsmmuParamsAt(old_s, params_ptr).reg_base as int)
                ))
        )) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (result.is_Err() ==> (
        GranuleAt(new_s, vsmmu_ptr).state == GranuleAt(old_s, vsmmu_ptr).state
        && RealmAt(new_s, rd).num_vsmmus == RealmAt(old_s, rd).num_vsmmus
    ))
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
        && (RmiVsmmuParamsAt(old_s, params_ptr).reg_top as int) > (RmiVsmmuParamsAt(old_s, params_ptr).reg_base as int))
        ==> (
            result.is_Ok()
            && GranuleAt(new_s, vsmmu_ptr).state == VSMMU
            && VsmmuAt(new_s, vsmmu_ptr).state == VSMMU_INACTIVE
            && VsmmuAt(new_s, vsmmu_ptr).realm == rd
            && VsmmuAt(new_s, vsmmu_ptr).reg_base == RmiVsmmuParamsAt(old_s, params_ptr).reg_base
            && VsmmuAt(new_s, vsmmu_ptr).reg_top == RmiVsmmuParamsAt(old_s, params_ptr).reg_top
            && VsmmuAt(new_s, vsmmu_ptr).aidr == RmiVsmmuParamsAt(old_s, params_ptr).aidr
            && VsmmuAt(new_s, vsmmu_ptr).idr == RmiVsmmuParamsAt(old_s, params_ptr).idr
            && (RealmAt(new_s, rd).num_vsmmus as int) == (RealmAt(old_s, rd).num_vsmmus as int) + 1
        ))
}
