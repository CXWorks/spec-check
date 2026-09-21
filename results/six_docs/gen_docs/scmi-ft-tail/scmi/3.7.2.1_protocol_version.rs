pub open spec fn 3.7.2.1_protocol_version_spec(result: Result<int32, ScmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result == Ok(0x30001))
}