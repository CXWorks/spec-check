pub open spec fn rmi_vsmmu_create_spec(
    result: RmiCommandReturnCode,
    rd: Address,
    vsmmu_ptr: Address,
    params_ptr: Address,
    old_s: S,
    new_s: S,
) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let realm = RealmAt(new_s, rd);
    let vsmmu = VsmmuAt(new_s, vsmmu_ptr);
    let params = RmiVsmmuParamsAt(old_s, params_ptr);

    (!ImplFeatures(old_s).feat_da == FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(
        old_s,
        rd,
    ).state != RD() ==> ResultEqual(result, RMI_ERROR_INPUT)) && (realm_pre.state != REALM_NEW()
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(old_s, vsmmu_ptr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegableDram(old_s, vsmmu_ptr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, vsmmu_ptr).state
        != DELEGATED() ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(
        old_s,
        params_ptr,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!GranuleAccessPermitted(
        old_s,
        params_ptr,
        PAS_NS(),
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!RmiVsmmuParamsIsValid(old_s, params_ptr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!AddrIsGranuleAligned(old_s, params.reg_base)
        || !AddrIsGranuleAligned(old_s, params.reg_top)) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && ((!AddrIsProtected(old_s, params.reg_base, realm_pre) || !AddrIsProtected(
        old_s,
        params.reg_top,
        realm_pre,
    ) || UInt(params.reg_top) <= UInt(params.reg_base)) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state == RD() && realm_pre.state
        == REALM_NEW() && AddrIsGranuleAligned(old_s, vsmmu_ptr) && PaIsDelegableDram(
        old_s,
        vsmmu_ptr,
    ) && GranuleAt(old_s, vsmmu_ptr).state == DELEGATED() && AddrIsGranuleAligned(old_s, params_ptr)
        && GranuleAccessPermitted(old_s, params_ptr, PAS_NS()) && RmiVsmmuParamsIsValid(
        old_s,
        params_ptr,
    ) && (AddrIsGranuleAligned(old_s, params.reg_base) && AddrIsGranuleAligned(
        old_s,
        params.reg_top,
    )) && (AddrIsProtected(old_s, params.reg_base, realm_pre) && AddrIsProtected(
        old_s,
        params.reg_top,
        realm_pre,
    ) && UInt(params.reg_top) > UInt(params.reg_base))) ==> (result == RMI_OK() && GranuleAt(
        new_s,
        vsmmu_ptr,
    ).state == VSMMU() && vsmmu.state == VSMMU_INACTIVE() && vsmmu.realm == rd && vsmmu.reg_base
        == params.reg_base && vsmmu.reg_top == params.reg_top && vsmmu.aidr == params.aidr
        && vsmmu.idr[0] == params.idr[0] && vsmmu.idr[1] == params.idr[1] && vsmmu.idr[2]
        == params.idr[2] && vsmmu.idr[3] == params.idr[3] && vsmmu.idr[4] == params.idr[4]
        && vsmmu.idr[5] == params.idr[5] && vsmmu.idr[6] == params.idr[6] && realm.num_vsmmus
        == realm_pre.num_vsmmus + 1))
}