pub proof fn rmi_data_destroy_rule (rd: Address, ipa: Address, result: Result<(), RmiStatusCode>, data: Address, top: Address, old_s: S, new_s: S)
    requires rmi_data_destroy_spec(rd, ipa, result, data, top, old_s, new_s),
{
  let old_walk = RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  let new_walk = RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  assert(result.is_Ok() ==> (old_walk.rtte.ripas != RAM || old_walk.rtte.ripas == RAM));
  assert(result.is_Ok() ==> (old_walk.rtte.state == ASSIGNED));
  assert((result.is_Ok() && old_walk.rtte.ripas != RAM) ==> (new_walk.rtte.ripas == old_walk.rtte.ripas));
  assert((result.is_Ok() && old_walk.rtte.ripas == RAM) ==> (new_walk.rtte.ripas == DESTROYED));
  assert(result.is_Ok() ==> (new_walk.rtte.state == UNASSIGNED));
}
