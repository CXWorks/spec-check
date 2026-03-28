pub proof fn rmi_vsmmu_map_rule (rd: Address, vsmmu_ptr: Address, ipa: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S)
    requires rmi_vsmmu_map_spec(rd, vsmmu_ptr, ipa, result, old_s, new_s),
{
  let old_walk = RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  let new_walk = RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  assert(result.is_Ok() ==> (old_walk.rtte.ripas == EMPTY));
  assert(result.is_Ok() ==> (old_walk.rtte.state == UNASSIGNED));
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == old_walk.rtte.ripas));
  assert(result.is_Ok() ==> (new_walk.rtte.state == ASSIGNED_VSMMU));
}
