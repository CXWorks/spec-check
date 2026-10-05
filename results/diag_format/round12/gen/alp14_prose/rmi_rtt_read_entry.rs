pub open spec fn rmi_rtt_read_entry_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, walk_level: UInt64, state: RmiRttEntryState, desc: Bits64, ripas: RmiRipas, old_s: S, new_s: S) -> bool {
  (RealmAt(old_s, rd) == RealmAt(new_s, rd))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY) == RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY))
  && (result.is_Ok() ==> walk_level == RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).level as UInt64)
  && ((!( (rd) % granule_size(old_s) == 0) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (is_delegable_physical_address(old_s, rd) ==> result.is_Ok())
    && (GranuleAt(old_s, rd) == GranuleAt(new_s, rd))
    && (is_valid_rtt_level(old_s, RealmAt(old_s, rd), level) ==> result.is_Ok())
    && (ipa % address_range_size(old_s, RealmAt(old_s, rd), level as int) == 0 ==> result.is_Ok())
    && (ipa >= pow2(RealmAt(old_s, rd).ipa_width as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (result.is_Err()
      ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY) == RttWalk(old_s, RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY))
  )
}