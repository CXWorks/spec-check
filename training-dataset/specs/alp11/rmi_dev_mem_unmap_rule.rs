pub proof fn rmi_dev_mem_unmap_rule (rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, pa: Address, top: Address, old_s: S, new_s: S)
    requires rmi_dev_mem_unmap_spec(rd, ipa, level, result, pa, top, old_s, new_s),
{
  let old_walk = RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int);
  let new_walk = RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int);
  assert(result.is_Ok() ==> (old_walk.rtte.ripas != DEV || old_walk.rtte.ripas == DEV));
  assert(result.is_Ok() ==> (old_walk.rtte.state == ASSIGNED_DEV));
  assert((result.is_Ok() && old_walk.rtte.ripas != DEV) ==> (new_walk.rtte.ripas == old_walk.rtte.ripas));
  assert((result.is_Ok() && old_walk.rtte.ripas == DEV) ==> (new_walk.rtte.ripas == DESTROYED));
  assert(result.is_Ok() ==> (new_walk.rtte.state == UNASSIGNED));
}
