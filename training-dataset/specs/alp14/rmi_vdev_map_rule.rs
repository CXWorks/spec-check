pub proof fn rmi_vdev_map_rule (rd: Address, vdev_ptr: Address, ipa: Address, level: Int64, addr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S)
    requires rmi_vdev_map_spec(rd, vdev_ptr, ipa, level, addr, result, old_s, new_s),
{
  let old_walk = RttWalk(old_s, RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int);
  let new_walk = RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY as int);
  assert(result.is_Ok() ==> (true));
  assert(result.is_Ok() ==> (old_walk.rtte.state == UNASSIGNED));
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == old_walk.rtte.ripas));
  assert(result.is_Ok() ==> (new_walk.rtte.state == ASSIGNED_DEV));
}
