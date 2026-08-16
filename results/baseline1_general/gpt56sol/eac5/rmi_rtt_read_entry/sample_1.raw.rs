pub open spec fn rmi_rtt_read_entry_spec(
    result: Result<(), RmiStatusCode>,
    rd: Address,
    ipa: Address,
    level: Int64,
    walk_level: UInt64,
    state: RmiRttEntryState,
    desc: Bits64,
    ripas: RmiRipas,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttLevelIsValid(old_s, rd, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ipa as int) >= (1 << Realm(old_s, rd).ipa_width) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (
        AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && Granule(old_s, rd).state == RD
        && RttLevelIsValid(old_s, rd, level as int)
        && AddrIsRttLevelAligned(old_s, ipa, level as int)
        && (ipa as int) < (1 << Realm(old_s, rd).ipa_width)
        ==> (
            result.is_Ok()
            && walk_level as int == RttWalk(new_s, rd, ipa).level
            && state == RttEntryState(new_s, RttWalk(new_s, rd, ipa).rtte.state)
            && (
                (RttWalk(new_s, rd, ipa).rtte.state == UNASSIGNED
                    || RttWalk(new_s, rd, ipa).rtte.state == UNASSIGNED_NS)
                ==> (
                    RttEntryFromDescriptor(new_s, desc).MemAttr == 0
                    && RttEntryFromDescriptor(new_s, desc).S2AP == 0
                    && RttEntryFromDescriptor(new_s, desc).SH == 0
                    && RttEntryFromDescriptor(new_s, desc).addr == 0
                )
            )
            && (
                (RttWalk(new_s, rd, ipa).rtte.state == ASSIGNED
                    || RttWalk(new_s, rd, ipa).rtte.state == TABLE)
                ==> (
                    RttEntryFromDescriptor(new_s, desc).MemAttr == 0
                    && RttEntryFromDescriptor(new_s, desc).S2AP == 0
                    && RttEntryFromDescriptor(new_s, desc).SH == 0
                    && RttEntryFromDescriptor(new_s, desc).addr
                        == RttWalk(new_s, rd, ipa).rtte.addr
                )
            )
            && (
                RttWalk(new_s, rd, ipa).rtte.state == ASSIGNED_NS
                ==> (
                    RttEntryFromDescriptor(new_s, desc).MemAttr
                        == RttWalk(new_s, rd, ipa).rtte.MemAttr
                    && RttEntryFromDescriptor(new_s, desc).S2AP
                        == RttWalk(new_s, rd, ipa).rtte.S2AP
                    && RttEntryFromDescriptor(new_s, desc).SH
                        == RttWalk(new_s, rd, ipa).rtte.SH
                    && RttEntryFromDescriptor(new_s, desc).addr
                        == RttWalk(new_s, rd, ipa).rtte.addr
                )
            )
            && (
                (RttWalk(new_s, rd, ipa).rtte.state != UNASSIGNED
                    && RttWalk(new_s, rd, ipa).rtte.state != ASSIGNED)
                ==> ripas == RMI_EMPTY
            )
        )
    )
    && new_s == old_s
}