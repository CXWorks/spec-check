pub open spec fn performance_domain_name_get__3_5_6_19_spec(domain_id: UInt32, result: Result<int32, [u8; 64]>, flags: UInt32, name: [u8; 64], old_s: S, new_s: S) -> bool {
  (result == SUCCESS ==> flags == 0)
  && (result == SUCCESS ==> name[0] == 0)
  && ((!(result == SUCCESS))
    ==> flags == 0)
  && ((!(result == SUCCESS))
    ==> name[0] == 0)
}