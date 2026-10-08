pub open spec fn rmi_rtt_read_entry_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, walk_level: UInt64, state: RmiRttEntryState, desc: Bits64, ripas: RmiRipas, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAt(old_s, rd).state == RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ipa >= pow2(RealmAt(old_s, rd).ipa_width as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (result.is_Ok() ==> (walk_level as int == level))
    && (result.is_Ok() ==> (state == RttEntryStateToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.state)))
    && (result.is_Ok() ==> (desc == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.s2ap_indirect.base_index as Bits64 | (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.s2ap_indirect.overlay_index as Bits64) << 64 | (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.s2ap_direct.read as Bits64) << 128 | (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.s2ap_direct.write as Bits64) << 136 | (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.attr_unprot as Bits64) << 144 | (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.addr as Bits64) << 152))
    && (result.is_Ok() ==> (ripas == RipasToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.state)))
}