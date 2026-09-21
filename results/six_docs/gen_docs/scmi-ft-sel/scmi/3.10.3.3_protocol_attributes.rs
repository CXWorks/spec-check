pub open spec fn 3.10.3.3_protocol_attributes_spec(result: int, attributes: uint32, old_s: S, new_s: S) -> bool {
  (result == SCMI_SUCCESS ==> attributes == 0)
}