pub proof fn rmi_rtt_fold_rule (rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, old_s: S, new_s: S)
    requires rmi_rtt_fold_spec(rd, ipa, level, result, rtt, old_s, new_s),
{
  let old_walk = RttWalk_(old_s, rd, ipa,level - 1 as int);
  let new_walk = RttWalk_(new_s, rd, ipa,level - 1 as int);
  // Unsupported
  // Unsupported
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == old_walk.rtte.ripas)); // XXX: FP
  assert(result.is_Ok() ==> (new_walk.rtte.state == old_walk.rtte.state)); // XXX: FP
}
