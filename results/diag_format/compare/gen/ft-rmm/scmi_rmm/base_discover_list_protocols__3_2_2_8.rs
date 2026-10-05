pub open spec fn base_discover_list_protocols__3_2_2_8_spec(skip: uint32, status: int32, num_protocols: uint32, protocols: [uint32; 1], old_s: S, new_s: S) -> bool {
  (!IsValidSkip(old_s, skip) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> ProtocolsPackedFourPerElement(protocols, num_protocols))
  && (ResultEqual(status, SUCCESS) ==> ProtocolsInAscendingOrder(protocols, num_protocols))
  && ((IsValidSkip(old_s, skip))
    ==> ResultEqual(status, SUCCESS))
}