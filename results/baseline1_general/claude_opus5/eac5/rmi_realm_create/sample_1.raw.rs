pub open spec fn rmi_realm_create_spec(result: Result<(), RmiStatusCode>, rd: Address, params_ptr: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RmiRealmParamsIsValid(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RealmParamsSupported(old_s, RealmParams(old_s, params_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (AddrInRange(old_s, rd, RealmParams(old_s, params_ptr).rtt_base,
            ((RealmParams(old_s, params_ptr).rtt_num_start as int) - 1) * (RMM_GRANULE_SIZE as int))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, rd).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsAligned(old_s, RealmParams(old_s, params_ptr).rtt_base,
            (RealmParams(old_s, params_ptr).rtt_num_start as int) * (RMM_GRANULE_SIZE as int))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttConfigIsValid(old_s,
            RealmParams(old_s, params_ptr).s2sz as int,
            RealmParams(old_s, params_ptr).rtt_level_start as int,
            RealmParams(old_s, params_ptr).rtt_num_start as int)
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttsStateEqual(RealmParams(old_s, params_ptr).rtt_base,
            RealmParams(old_s, params_ptr).rtt_num_start as int, DELEGATED)
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!VmidIsValid(old_s, RealmParams(old_s, params_ptr).vmid)
            || !VmidIsFree(old_s, RealmParams(old_s, params_ptr).vmid))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((AddrIsGranuleAligned(old_s, params_ptr)
        && PaIsDelegable(old_s, params_ptr)
        && GranuleAccessPermitted(old_s, params_ptr, PAS_NS)
        && RmiRealmParamsIsValid(old_s, params_ptr)
        && RealmParamsSupported(old_s, RealmParams(old_s, params_ptr))
        && !AddrInRange(old_s, rd, RealmParams(old_s, params_ptr).rtt_base,
                ((RealmParams(old_s, params_ptr).rtt_num_start as int) - 1) * (RMM_GRANULE_SIZE as int))
        && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && Granule(old_s, rd).state == DELEGATED
        && AddrIsAligned(old_s, RealmParams(old_s, params_ptr).rtt_base,
                (RealmParams(old_s, params_ptr).rtt_num_start as int) * (RMM_GRANULE_SIZE as int))
        && RttConfigIsValid(old_s,
                RealmParams(old_s, params_ptr).s2sz as int,
                RealmParams(old_s, params_ptr).rtt_level_start as int,
                RealmParams(old_s, params_ptr).rtt_num_start as int)
        && RttsStateEqual(RealmParams(old_s, params_ptr).rtt_base,
                RealmParams(old_s, params_ptr).rtt_num_start as int, DELEGATED)
        && VmidIsValid(old_s, RealmParams(old_s, params_ptr).vmid)
        && VmidIsFree(old_s, RealmParams(old_s, params_ptr).vmid))
        ==> (result.is_Ok()
            && Granule(new_s, rd).state == RD
            && Realm(new_s, rd).state == REALM_NEW
            && Realm(new_s, rd).rec_index == 0
            && Realm(new_s, rd).rtt_base == RealmParams(old_s, params_ptr).rtt_base
            && RttsStateEqual(Realm(new_s, rd).rtt_base, Realm(new_s, rd).rtt_num_start as int, RTT)
            && RttsAllProtectedEntriesState(new_s, Realm(new_s, rd).rtt_base,
                    Realm(new_s, rd).rtt_num_start as int, UNASSIGNED)
            && RttsAllUnprotectedEntriesState(new_s, Realm(new_s, rd).rtt_base,
                    Realm(new_s, rd).rtt_num_start as int, UNASSIGNED_NS)
            && RttsAllProtectedEntriesRipas(new_s, Realm(new_s, rd).rtt_base,
                    Realm(new_s, rd).rtt_num_start as int, EMPTY)
            && Realm(new_s, rd).ipa_width == RealmParams(old_s, params_ptr).s2sz as int
            && Equal(Realm(new_s, rd).hash_algo, RealmParams(old_s, params_ptr).hash_algo)
            && Realm(new_s, rd).measurements[0] == RimInit(new_s, Realm(new_s, rd).hash_algo,
                    RealmParams(old_s, params_ptr))
            && Realm(new_s, rd).rtt_level_start == RealmParams(old_s, params_ptr).rtt_level_start as int
            && Realm(new_s, rd).rtt_num_start == RealmParams(old_s, params_ptr).rtt_num_start as int
            && Realm(new_s, rd).vmid == RealmParams(old_s, params_ptr).vmid
            && Realm(new_s, rd).rpv == RealmParams(old_s, params_ptr).rpv))
}