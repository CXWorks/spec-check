pub proof fn rmi_vdev_validate_mapping_rule (rd: Address, rec_ptr: Address, pdev_ptr: Address, vdev_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S)
    requires rmi_vdev_validate_mapping_spec(rd, rec_ptr, pdev_ptr, vdev_ptr, base, top, result, out_top, old_s, new_s),
{
  let old_walk = RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  let new_walk = RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int);
  assert(result.is_Ok() ==> (true));
  // Unsupported
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == DEV));
  assert(result.is_Ok() ==> (new_walk.rtte.state == old_walk.rtte.state));
}
