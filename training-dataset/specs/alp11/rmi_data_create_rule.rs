pub proof fn rmi_data_create_rule (rd: Address, data: Address, ipa: Address, src: Address, flags: RmiDataFlags, result: Result<(), RmiStatusCode>, old_s: S, new_s: S)
    requires rmi_data_create_spec(rd, data, ipa, src, flags, result, old_s, new_s),
{
  let old_walk = RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  let new_walk = RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  assert(result.is_Ok() ==> (true));
  assert(result.is_Ok() ==> (old_walk.rtte.state == UNASSIGNED));
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == RAM));
  assert(result.is_Ok() ==> (new_walk.rtte.state == ASSIGNED));
}
