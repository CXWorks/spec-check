pub open spec fn 3.2.2.3_protocol_attributes_spec(attributes: UInt32, result: Result<int32, RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Ok() ==> (RealmAt(new_s, 0).protocol_attributes == attributes))
}