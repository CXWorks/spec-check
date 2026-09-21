pub open spec fn 3.10.3.3_protocol_attributes_spec(result: Result<int32, RsiCommandReturnCode>, attributes: uint32, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> attributes == 0)
}