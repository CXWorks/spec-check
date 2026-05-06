pub open spec fn rmi_rtt_aux_create_spec(
    rd: Address,
    rtt: Address,
    ipa: Address,
    level: Int64,
    index: UInt64,
    result: Result<(), RmiStatusCode>,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        old_s,
        rd,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, rd).state != RD
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!RttLevelIsValid(
        old_s,
        RealmAt(old_s, rd),
        level as int,
    ) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int)) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY
        || index > RealmAt(old_s, rd).num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !AddrIsGranuleAligned(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !PaIsDelegableDram(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(
        old_s,
        rtt,
    ).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT)) && (((RealmAt(
        old_s,
        rd,
    ).feat_lpa2 == FEATURE_FALSE) && ((rtt) >= 2 ^ 48)) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, index as int).level < level
        - 1 ==> ResultEqual(
        result,
        RMI_ERROR_RTT_AUX(
            RttWalk(new_s, RealmAt(new_s, rd), ipa, level - 1 as int, index as int).level as int,
        ),
    )) && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, index as int).rtte.state
        == TABLE ==> ResultEqual(
        result,
        RMI_ERROR_RTT_AUX(
            RttWalk(new_s, RealmAt(new_s, rd), ipa, level - 1 as int, index as int).level as int,
        ),
    )) && (result.is_Ok() ==> GranuleAt(new_s, rtt).state == RTT) && (result.is_Ok() ==> RttWalk(
        new_s,
        RealmAt(new_s, rd),
        ipa,
        level - 1 as int,
        index as int,
    ).rtte.state == TABLE) && (result.is_Ok() ==> RttWalk(
        new_s,
        RealmAt(new_s, rd),
        ipa,
        level - 1 as int,
        index as int,
    ).rtte.addr == rtt) && (result.is_Ok() && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))
        ==> RttAllEntriesRipas(
        new_s,
        RttAt(new_s, rtt),
        RttWalk(new_s, RealmAt(new_s, rd), ipa, level - 1 as int, index as int).rtte.ripas,
    )) && (result.is_Ok() ==> RttAllEntriesState(
        new_s,
        RttAt(new_s, rtt),
        RttWalk(new_s, RealmAt(new_s, rd), ipa, level - 1 as int, index as int).rtte.state,
    )) && (result.is_Ok() && RttWalk(
        old_s,
        RealmAt(old_s, rd),
        ipa,
        level - 1 as int,
        index as int,
    ).rtte.state != UNASSIGNED ==> RttAllEntriesContiguous(
        new_s,
        RttAt(new_s, rtt),
        RttWalk(new_s, RealmAt(new_s, rd), ipa, level - 1 as int, index as int).rtte.addr,
        level as int,
    )) && ((AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && !(GranuleAt(
        old_s,
        rd,
    ).state != RD) && !((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int)
        || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int))) && AddrIsRttLevelAligned(
        old_s,
        ipa,
        level - 1 as int,
    ) && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) && !((RealmAt(old_s, rd).rtt_tree_per_plane
        == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(
        old_s,
        rd,
    ).num_aux_planes)) && AddrIsGranuleAligned(old_s, rtt) && PaIsDelegableDram(old_s, rtt) && !(
    GranuleAt(old_s, rtt).state != DELEGATED) && !(((RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE)
        && ((rtt) >= 2 ^ 48))) && !(RttWalk(
        old_s,
        RealmAt(old_s, rd),
        ipa,
        level - 1 as int,
        index as int,
    ).level < level - 1) && !(RttWalk(
        old_s,
        RealmAt(old_s, rd),
        ipa,
        level - 1 as int,
        index as int,
    ).rtte.state == TABLE)) ==> result.is_Ok()) && (result.is_Err() ==> GranuleAt(new_s, rtt).state
        == GranuleAt(old_s, rtt).state) && (result.is_Err() ==> RttWalk(
        new_s,
        RealmAt(new_s, rd),
        ipa,
        level - 1 as int,
        index as int,
    ).rtte.state == RttWalk(
        old_s,
        RealmAt(old_s, rd),
        ipa,
        level - 1 as int,
        index as int,
    ).rtte.state) && (result.is_Err() ==> RttWalk(
        new_s,
        RealmAt(new_s, rd),
        ipa,
        level - 1 as int,
        index as int,
    ).rtte.addr == RttWalk(
        old_s,
        RealmAt(old_s, rd),
        ipa,
        level - 1 as int,
        index as int,
    ).rtte.addr) && (RttWalk(
        new_s,
        RealmAt(new_s, rd),
        ipa,
        level - 1 as int,
        index as int,
    ).rtte.ripas == RttWalk(
        old_s,
        RealmAt(old_s, rd),
        ipa,
        level - 1 as int,
        index as int,
    ).rtte.ripas)
}