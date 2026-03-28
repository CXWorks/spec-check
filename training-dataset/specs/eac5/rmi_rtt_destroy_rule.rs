pub proof fn rmi_rtt_destroy_rule (rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, top: Address, old_s: S, new_s: S)
    requires rmi_rtt_destroy_spec(rd, ipa, level, result, rtt, top, old_s, new_s),
{
  let old_walk = RttWalk_(old_s, rd, ipa,level - 1 as int);
  let new_walk = RttWalk_(new_s, rd, ipa,level - 1 as int);
  assert(result.is_Ok() ==> (true));
  // Unsupported
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == DESTROYED));
  assert(result.is_Ok() ==> (new_walk.rtte.state == old_walk.rtte.state)); // XXX: TP
}
