pub open spec fn rmi_realm_create_spec(
    rd: Address,
    params_ptr: Address,
    result: Result<(), RmiStatusCode>,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !PaIsDelegable(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (!RmiRealmParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !RealmParamsSupported(old_s, RealmParams(old_s, params_ptr)) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (AddrInRange(
        old_s,
        rd,
        RealmParams(old_s, params_ptr).rtt_base,
        (RealmParams(old_s, params_ptr).rtt_num_start - 1) * RMM_GRANULE_SIZE,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(old_s, rd)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(old_s, rd) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (Granule(old_s, rd).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !AddrIsAligned(
        old_s,
        RealmParams(old_s, params_ptr).rtt_base,
        RealmParams(old_s, params_ptr).rtt_num_start * RMM_GRANULE_SIZE,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!RttConfigIsValid(
        old_s,
        RealmParams(old_s, params_ptr).s2sz as int,
        RealmParams(old_s, params_ptr).rtt_level_start as int,
        RealmParams(old_s, params_ptr).rtt_num_start as int,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!RttsStateEqual(
        RealmParams(old_s, params_ptr).rtt_base,
        RealmParams(old_s, params_ptr).rtt_num_start as int,
        DELEGATED,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!VmidIsValid(
        old_s,
        RealmParams(old_s, params_ptr).vmid,
    ) || !VmidIsFree(old_s, RealmParams(old_s, params_ptr).vmid) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (result.is_Ok() ==> Granule(new_s, rd).state == RD) && (result.is_Ok() ==> Realm(
        new_s,
        rd,
    ).state == REALM_NEW) && (result.is_Ok() ==> Realm(new_s, rd).rec_index == 0) && (result.is_Ok()
        ==> Realm(new_s, rd).rtt_base == RealmParams(new_s, params_ptr).rtt_base) && (result.is_Ok()
        ==> RttsStateEqual(Realm(new_s, rd).rtt_base, Realm(new_s, rd).rtt_num_start as int, RTT))
        && (result.is_Ok() ==> RttsAllProtectedEntriesState(
        new_s,
        Realm(new_s, rd).rtt_base,
        Realm(new_s, rd).rtt_num_start as int,
        UNASSIGNED,
    )) && (result.is_Ok() ==> RttsAllUnprotectedEntriesState(
        new_s,
        Realm(new_s, rd).rtt_base,
        Realm(new_s, rd).rtt_num_start as int,
        UNASSIGNED_NS,
    )) && (result.is_Ok() ==> RttsAllProtectedEntriesRipas(
        new_s,
        Realm(new_s, rd).rtt_base,
        Realm(new_s, rd).rtt_num_start as int,
        EMPTY,
    )) && (result.is_Ok() ==> Realm(new_s, rd).ipa_width == RealmParams(new_s, params_ptr).s2sz)
        && (result.is_Ok() ==> Equal(
        Realm(new_s, rd).hash_algo,
        RealmParams(new_s, params_ptr).hash_algo,
    )) && (result.is_Ok() ==> Realm(new_s, rd).measurements[0] == RimInit(
        new_s,
        Realm(new_s, rd).hash_algo,
        RealmParams(new_s, params_ptr),
    ))
    //&& (result.is_Ok() ==> (Realm(new_s, rd).measurements[1] == 0 && Realm(new_s, rd).measurements[2] == 0 && Realm(new_s, rd).measurements[3] == 0 && Realm(new_s, rd).measurements[4] == 0))
     && (result.is_Ok() ==> Realm(new_s, rd).rtt_level_start == RealmParams(
        new_s,
        params_ptr,
    ).rtt_level_start) && (result.is_Ok() ==> Realm(new_s, rd).rtt_num_start == RealmParams(
        new_s,
        params_ptr,
    ).rtt_num_start) && (result.is_Ok() ==> Realm(new_s, rd).vmid == RealmParams(
        new_s,
        params_ptr,
    ).vmid) && (result.is_Ok() ==> Realm(new_s, rd).rpv == RealmParams(new_s, params_ptr).rpv) && ((
    AddrIsGranuleAligned(old_s, params_ptr) && PaIsDelegable(old_s, params_ptr)
        && GranuleAccessPermitted(old_s, params_ptr, PAS_NS) && RmiRealmParamsIsValid(
        old_s,
        params_ptr,
    ) && RealmParamsSupported(old_s, RealmParams(old_s, params_ptr)) && !(AddrInRange(
        old_s,
        rd,
        RealmParams(old_s, params_ptr).rtt_base,
        (RealmParams(old_s, params_ptr).rtt_num_start - 1) * RMM_GRANULE_SIZE,
    )) && AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && !(Granule(old_s, rd).state
        != DELEGATED) && AddrIsAligned(
        old_s,
        RealmParams(old_s, params_ptr).rtt_base,
        RealmParams(old_s, params_ptr).rtt_num_start * RMM_GRANULE_SIZE,
    ) && RttConfigIsValid(
        old_s,
        RealmParams(old_s, params_ptr).s2sz as int,
        RealmParams(old_s, params_ptr).rtt_level_start as int,
        RealmParams(old_s, params_ptr).rtt_num_start as int,
    ) && RttsStateEqual(
        RealmParams(old_s, params_ptr).rtt_base,
        RealmParams(old_s, params_ptr).rtt_num_start as int,
        DELEGATED,
    ) && VmidIsValid(old_s, RealmParams(old_s, params_ptr).vmid) || !VmidIsFree(
        old_s,
        RealmParams(old_s, params_ptr).vmid,
    )) ==> result.is_Ok()) && (result.is_Err() ==> Granule(new_s, rd).state == Granule(
        old_s,
        rd,
    ).state) && (result.is_Err() ==> Realm(new_s, rd).state == Realm(old_s, rd).state) && (
    result.is_Err() ==> Realm(new_s, rd).rec_index == Realm(old_s, rd).rec_index) && (
    result.is_Err() ==> Realm(new_s, rd).rtt_base == Realm(old_s, rd).rtt_base) && (result.is_Err()
        ==> Realm(new_s, rd).ipa_width == Realm(old_s, rd).ipa_width) && (result.is_Err() ==> Realm(
        new_s,
        rd,
    ).measurements[0] == Realm(old_s, rd).measurements[0]) && (result.is_Err() ==> Realm(
        new_s,
        rd,
    ).rtt_level_start == Realm(old_s, rd).rtt_level_start) && (result.is_Err() ==> Realm(
        new_s,
        rd,
    ).rtt_num_start == Realm(old_s, rd).rtt_num_start) && (result.is_Err() ==> Realm(new_s, rd).vmid
        == Realm(old_s, rd).vmid) && (result.is_Err() ==> Realm(new_s, rd).rpv == Realm(
        old_s,
        rd,
    ).rpv)
}