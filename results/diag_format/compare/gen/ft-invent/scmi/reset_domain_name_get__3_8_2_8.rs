pub open spec fn reset_domain_name_get__3_8_2_8_spec(domain_id: UInt32, result: Result<int32, ()>, flags: UInt32, name: [uint8; 64], old_s: S, new_s: S) -> bool {
  (result == NOT_FOUND ==> flags == 0)
  && (result == SUCCESS ==> flags == 0)
  && ((!(result == NOT_FOUND) &&
       !(result == SUCCESS))
    ==> flags == 0)
}