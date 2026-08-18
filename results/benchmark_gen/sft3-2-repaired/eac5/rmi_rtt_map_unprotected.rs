pub open spec fn rmi_rtt_map_unprotected_spec(rd: Address, ipa: Address, level: int, desc: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!RttDescriptorIsValidForUnprotected(old_s, desc) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RttLevelIsBlockOrPage(old_s, rd, level) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, RttEntryFromDescriptor(old_s, desc).addr, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, ipa, Realm(old_s, rd))) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, rd, ipa, level as int).level < level ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, rd, ipa, level as int).level as int)))
  && (RttWalk(old_s, rd, ipa, level as int).rtte.state != UNASSIGNED_NS ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, rd, ipa, level as int).level as int)))
  && (result.is_Ok() ==> RttWalk(new_s, rd, ipa, level as int).rtte.state == ASSIGNED_NS)
  && (result.is_Ok() ==> (RttWalk(new_s, rd, ipa, level as int).rtte.MemAttr == RttEntryFromDescriptor(new_s, desc).MemAttr && RttWalk(new_s, rd, ipa, level as int).rtte.S2AP == RttEntryFromDescriptor(new_s, desc).S2AP && RttWalk(new_s, rd, ipa, level as int).rtte.SH == RttEntryFromDescriptor(new_s, desc).SH && RttWalk(new_s, rd, ipa, level as int).rtte.addr == RttEntryFromDescriptor(new_s, desc).addr))
  && ((!(RttDescriptorIsValidForUnprotected(old_s, desc)) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       RttLevelIsBlockOrPage(old_s, rd, level) &&
       AddrIsRttLevelAligned(old_s, RttEntryFromDescriptor(old_s, desc).addr, level as int) &&
       AddrIsRttLevelAligned(old_s, ipa, level as int) &&
       !(((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, ipa, Realm(old_s, rd)))) &&
       !(RttWalk(old_s, rd, ipa, level as int).level < level) &&
       !(RttWalk(old_s, rd, ipa, level as int).rtte.state != UNASSIGNED_NS))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, rd, ipa, level as int).rtte.state == RttWalk(old_s, rd, ipa, level as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, rd, ipa, level as int).rtte.MemAttr == RttWalk(old_s, rd, ipa, level as int).rtte.MemAttr)
  && (result.is_Err()
    ==> RttWalk(new_s, rd, ipa, level as int).rtte.S2AP == RttWalk(old_s, rd, ipa, level as int).rtte.S2AP)
  && (result.is_Err()
    ==> RttWalk(new_s, rd, ipa, level as int).rtte.SH == RttWalk(old_s, rd, ipa, level as int).rtte.SH)
  && (result.is_Err()
    ==> RttWalk(new_s, rd, ipa, level as int).rtte.addr == RttWalk(old_s, rd, ipa, level as int).rtte.addr)
  && (RttWalk(new_s, rd, ipa, level as int).rtte.pte == RttWalk(old_s, rd, ipa, level as int).rtte.pte)
}