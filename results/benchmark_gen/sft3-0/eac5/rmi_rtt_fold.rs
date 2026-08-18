pub open spec fn rmi_rtt_fold_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!RttLevelIsValid(old_s, rd, level as int) || RttLevelIsStarting(old_s, rd, level as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, rd, ipa,level - 1 as int).level < level - 1 ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, rd, ipa,level - 1 as int).level as int)))
  && (RttWalk(old_s, rd, ipa,level - 1 as int).rtte.state != TABLE ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, rd, ipa,level - 1 as int).level as int)))
  && (!RttIsHomogeneous(old_s, Rtt(RttWalk(old_s, rd, ipa,level - 1 as int).rtte.addr)) ==> ResultEqual(result, RMI_ERROR_RTT(level as int)))
  && (result.is_Ok() ==> RttWalk(new_s, rd, ipa,level - 1 as int).rtte.state == RttFold(new_s, Rtt(RttWalk(new_s, rd, ipa,level - 1 as int).rtte.addr)).state)
  && (result.is_Ok() && (RttFold(old_s, Rtt(RttWalk(old_s, rd, ipa,level - 1 as int).rtte.addr)).state != UNASSIGNED && RttFold(old_s, Rtt(RttWalk(old_s, rd, ipa,level - 1 as int).rtte.addr)).state != UNASSIGNED_NS) ==> RttWalk(new_s, rd, ipa,level - 1 as int).rtte.addr == RttFold(new_s, Rtt(RttWalk(new_s, rd, ipa,level - 1 as int).rtte.addr)).addr)
  && (result.is_Ok() && (RttFold(old_s, Rtt(RttWalk(old_s, rd, ipa,level - 1 as int).rtte.addr)).state == ASSIGNED || RttFold(old_s, Rtt(RttWalk(old_s, rd, ipa,level - 1 as int).rtte.addr)).state == ASSIGNED_NS) ==> (RttWalk(new_s, rd, ipa,level - 1 as int).rtte.MemAttr == RttFold(new_s, Rtt(RttWalk(new_s, rd, ipa,level - 1 as int).rtte.addr)).MemAttr && RttWalk(new_s, rd, ipa,level - 1 as int).rtte.S2AP == RttFold(new_s, Rtt(RttWalk(new_s, rd, ipa,level - 1 as int).rtte.addr)).S2AP && RttWalk(new_s, rd, ipa,level - 1 as int).rtte.SH == RttFold(new_s, Rtt(RttWalk(new_s, rd, ipa,level - 1 as int).rtte.addr)).SH))
  && (result.is_Ok() && AddrIsProtected(old_s, ipa, Realm(old_s, rd)) ==> RttWalk(new_s, rd, ipa,level - 1 as int).rtte.ripas == RttFold(new_s, Rtt(RttWalk(new_s, rd, ipa,level - 1 as int).rtte.addr)).ripas)
  && (result.is_Ok() ==> Granule(new_s, RttWalk(new_s, rd, ipa,level - 1 as int).rtte.addr).state == DELEGATED)
  && (result.is_Ok() ==> rtt == RttWalk(new_s, rd, ipa,level - 1 as int).rtte.addr)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       !((!RttLevelIsValid(old_s, rd, level as int) || RttLevelIsStarting(old_s, rd, level as int))) &&
       AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) &&
       !((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat)) &&
       !(RttWalk(old_s, rd, ipa,level - 1 as int).level < level - 1) &&
       !(RttWalk(old_s, rd, ipa,level - 1 as int).rtte.state != TABLE) &&
       RttIsHomogeneous(old_s, Rtt(RttWalk(old_s, rd, ipa,level - 1 as int).rtte.addr)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, rd, ipa,level - 1 as int).rtte.state == RttWalk(old_s, rd, ipa,level - 1 as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, rd, ipa,level - 1 as int).rtte.addr == RttWalk(old_s, rd, ipa,level - 1 as int).rtte.addr)
  && (result.is_Err()
    ==> RttWalk(new_s, rd, ipa,level - 1 as int).rtte.ripas == RttWalk(old_s, rd, ipa,level - 1 as int).rtte.ripas)
  && (result.is_Err()
    ==> Granule(new_s, RttWalk(new_s, rd, ipa,level - 1 as int).rtte.addr).state == Granule(old_s, RttWalk(old_s, rd, ipa,level - 1 as int).rtte.addr).state)
  && (result.is_Err()
    ==> RttWalk(new_s, rd, ipa,level - 1 as int).rtte.ripas == RttWalk(old_s, rd, ipa,level - 1 as int).rtte.ripas))
}