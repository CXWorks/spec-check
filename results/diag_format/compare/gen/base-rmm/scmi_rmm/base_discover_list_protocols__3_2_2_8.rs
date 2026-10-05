pub open spec fn base_discover_list_protocols__3_2_2_8_spec(
    result: int32,
    num_protocols: uint32,
    protocols: uint32[1 + (num_protocols - 1) / 4],
    skip: uint32,
    old_s: S,
    new_s: S
) -> bool {
    (!IsValidSkip(skip) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, SUCCESS) ==> (
        ResultEqual(result, SUCCESS)
        && protocols.contains(num_protocols)
        && ProtocolsPackedFourPerElement(protocols, num_protocols)
        && ProtocolsInAscendingOrder(protocols, num_protocols)
    ))
}