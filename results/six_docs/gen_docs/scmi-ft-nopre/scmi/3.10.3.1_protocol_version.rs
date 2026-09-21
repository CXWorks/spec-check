pub open spec fn 3.10.3.1_protocol_version_spec(result: Result<int32, RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Ok() ==> result.unwrap() == 0x30000)
}