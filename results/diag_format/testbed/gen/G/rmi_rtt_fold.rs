pub open spec fn rmi_rtt_fold_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, old_s: S, new_s: S) -> bool {
  ((!(AddrIsGranuleAligned(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (!(PaIsDelegable(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (!(GranuleAt(old_s, rd).state == RD) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (!(RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (!(RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (!(AddrIsRttLevelAligned(old_s, ipa, (level - 1) as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (!(ipa < pow2(RealmAt(old_s, rd).ipa_width as nat)) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (result.is_Ok() ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == DELEGATED)
   && (result.is_Ok() && (RttWalk(old_s, RealmAt(old_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED && RttWalk(old_s, RealmAt(old_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED_NS) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == rtt)
   && (result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_prot == RTT_PROTECTED)
   && (result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.s2ap_indirect.encoding == S2AP_INDIRECT)
   && (result.is_Ok() && is_protected_ipa(old_s, ipa, RealmAt(old_s, rd)) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
   && ((result.is_Ok() && !(AddrIsGranuleAligned(old_s, rd)) ||
        result.is_Ok() && !(PaIsDelegable(old_s, rd)) ||
        result.is_Ok() && !(GranuleAt(old_s, rd).state == RD) ||
        result.is_Ok() && !(RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int)) ||
        result.is_Ok() && !(RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int)) ||
        result.is_Ok() && !(AddrIsRttLevelAligned(old_s, ipa, (level - 1) as int)) ||
        result.is_Ok() && !(ipa < pow2(RealmAt(old_s, rd).ipa_width as nat)))
     ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
   && (result.is_Err()
     ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
   && (result.is_Err()
     ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == RttWalk(old_s, RealmAt(old_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
   && (result.is_Err()
     ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_prot == RttWalk(old_s, RealmAt(old_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_prot)
   && (result.is_Err()
     ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.s2ap_indirect.encoding == RttWalk(old_s, RealmAt(old_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.s2ap_indirect.encoding)
   && (result.is_Err()
     ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,(level - 1) as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
  )
}

pub open spec fn is_protected_ipa(s: S, ipa: Address, realm: RmmRealm) -> bool {
  RttWalk(s, realm, ipa,RttLevelIsStarting(s, realm, 0) as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED
}