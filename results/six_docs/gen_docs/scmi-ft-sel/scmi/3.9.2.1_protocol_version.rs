pub open spec fn 3.9.2.1_protocol_version_spec(result: Result<int, ScmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result == Ok(0x20001))
}