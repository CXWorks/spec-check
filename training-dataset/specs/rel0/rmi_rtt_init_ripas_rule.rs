pub proof fn rmi_rtt_init_ripas_rule (rd: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S)
    requires rmi_rtt_init_ripas_spec(rd, base, top, result, out_top, old_s, new_s),
{
  let old_walk = RttWalk_(old_s, rd,base,RMM_RTT_PAGE_LEVEL as int);
  let new_walk = RttWalk_(new_s, rd,base,RMM_RTT_PAGE_LEVEL as int);
  assert(result.is_Ok() ==> (true));
  assert(result.is_Ok() ==> (old_walk.rtte.state == UNASSIGNED));
  assert(result.is_Ok() ==> (new_walk.rtte.ripas == RAM)); // XXX: FP
  assert(result.is_Ok() ==> (new_walk.rtte.state == old_walk.rtte.state)); // XXX: FP
}
