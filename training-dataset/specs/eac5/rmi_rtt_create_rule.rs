pub proof fn rmi_rtt_create_rule (rd: Address, rtt: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S)
    requires rmi_rtt_create_spec(rd, rtt, ipa, level, result, old_s, new_s),
{
  let old_walk = RttWalk_(old_s, rd, ipa,level - 1 as int);
  let new_walk = RttWalk_(new_s, rd, ipa,level - 1 as int);
  assert(result.is_Ok() ==> (true));
  assert(result.is_Ok() ==> (true));
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == old_walk.rtte.ripas));
  assert(result.is_Ok() ==> (new_walk.rtte.state == old_walk.rtte.state)); // XXX: FP
}
