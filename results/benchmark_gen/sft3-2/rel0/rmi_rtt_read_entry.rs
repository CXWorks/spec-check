pub open spec fn rmi_rtt_read_entry_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, walk_level: UInt64, state: RmiRttEntryState, desc: Bits64, ripas: RmiRipas, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RttLevelIsValid(old_s, rd, level) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> state == RttEntryFromDescriptor(new_s, RttWalk(new_s,rd, ipa, level as int).rtte.state))
  && (result.is_Ok() && (RttWalk(old_s, rd, ipa, level as int).rtte.state == UNASSIGNED || RttWalk(old_s, rd, ipa, level as int).rtte.state == UNASSIGNED_NS) ==> (RttEntryFromDescriptor(new_s, RttWalk(new_s,rd, ipa, level as int).rtte).MemAttr == 0 && RttEntryFromDescriptor(new_s, RttWalk(new_s,rd, ipa, level as int).rtte).S2AP == 0 && RttEntryFromDescriptor(new_s, RttWalk(new_s,rd, ipa, level as int).rtte).addr == 0))
  && (result.is_Ok() && (RttWalk(old_s, rd, ipa, level as int).rtte.state == ASSIGNED || RttWalk(old_s, rd, ipa, level as int).rtte.state == TABLE) ==> (RttEntryFromDescriptor(new_s, RttWalk(new_s,rd, ipa, level as int).rtte).MemAttr == 0 && RttEntryFromDescriptor(new_s, RttWalk(new_s,rd, ipa, level as int).rtte).S2AP == 0 && RttEntryFromDescriptor(new_s, RttWalk(new_s,rd, ipa, level as int).rtte).addr == RttWalk(new_s, rd, ipa, level as int).rtte.addr))
  && (result.is_Ok() && RttWalk(old_s, rd, ipa, level as int).rtte.state == ASSIGNED_NS ==> (RttEntryFromDescriptor(new_s, RttWalk(new_s,rd, ipa, level as int).rtte).MemAttr == RttWalk(new_s, rd, ipa, level as int).rtte.MemAttr && RttEntryFromDescriptor(new_s, RttWalk(new_s,rd, ipa, level as int).rtte).S2AP == RttWalk(new_s, rd, ipa, level as int).rtte.S2AP && RttEntryFromDescriptor(new_s, RttWalk(new_s,rd, ipa, level as int).rtte).addr == RttWalk(new_s, rd, ipa, level as int).rtte.addr))
  && (result.is_Ok() && (RttWalk(old_s, rd, ipa, level as int).rtte.state == UNASSIGNED || RttWalk(old_s, rd, ipa, level as int).rtte.state == ASSIGNED) ==> ripas == RipasToRmi(new_s, RttWalk(new_s,rd, ipa, level as int).rtte.ripas))
  && (result.is_Ok() && (RttWalk(old_s, rd, ipa, level as int).rtte.state == UNASSIGNED_NS || RttWalk(old_s, rd, ipa, level as int).rtte.state == ASSIGNED_NS) ==> ripas == RMI_EMPTY)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       RttLevelIsValid(old_s, rd, level) &&
       AddrIsRttLevelAligned(old_s, ipa, level as int) &&
       !((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttEntryFromDescriptor(new_s, RttWalk(new_s,rd, ipa, level as int).rtte).MemAttr == RttEntryFromDescriptor(old_s, RttWalk(old_s,rd, ipa, level as int).rtte).MemAttr)
  && (result.is_Err()
    ==> RttEntryFromDescriptor(new_s, RttWalk(new_s,rd, ipa, level as int).rtte).S2AP == RttEntryFromDescriptor(old_s, RttWalk(old_s,rd, ipa, level as int).rtte).S2AP)
  && (result.is_Err()
    ==> RttEntryFromDescriptor(new_s, RttWalk(new_s,rd, ipa, level as int).rtte).addr == RttEntryFromDescriptor(old_s, RttWalk(old_s,rd, ipa, level as int).rtte).addr)
  && (result.is_Err()
    ==> RttWalk(new_s, rd, ipa, level as int).rtte.ripas == RttWalk(old_s, rd, ipa, level as int).rtte.ripas)
}