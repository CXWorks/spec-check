pub proof fn rmi_rtt_fold_rule (rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, old_s: S, new_s: S)
    requires rmi_rtt_fold_spec(rd, ipa, level, result, rtt, old_s, new_s),
{
  let old_walk = RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int);
  let new_walk = RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int);
  // Unsupported
  // Unsupported
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == old_walk.rtte.ripas));
  assert(result.is_Ok() ==> (new_walk.rtte.state == old_walk.rtte.state));
}
