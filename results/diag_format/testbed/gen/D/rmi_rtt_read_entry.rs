pub open spec fn rmi_rtt_read_entry_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, walk_level: UInt64, state: RmiRttEntryState, desc: Bits64, ripas: RmiRipas, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttLevelIsValid(old_s, RealmAt(old_s, rd), level) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsAligned(old_s, ipa, RttLevelSize(level as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ipa >= pow2(old_s, RealmAt(old_s, rd).ipa_width) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (result.is_Ok() ==> (walk_level as int == level) && (state == RttEntryStateToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.state)) && (desc == RttDescriptorDecode(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte, RealmAt(old_s, rd).rtt_s2ap_encoding)) && (ripas == RmiRipasToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.ripas)))
}