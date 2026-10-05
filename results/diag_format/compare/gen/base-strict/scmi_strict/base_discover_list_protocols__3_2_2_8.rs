pub open spec fn base_discover_list_protocols__3_2_2_8_spec(result: Int32, num_protocols: UInt32, protocols: UInt32, old_s: S, new_s: S) -> bool {
    (!IsValidSkip(skip) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, SUCCESS) ==> (
        ResultEqual(result, SUCCESS)
        && (forall|i: UInt32| i < num_protocols ==> ProtocolIdAt(protocols, i) == AccessibleProtocolAt(skip + i))
        && (forall|i: UInt32| i < num_protocols ==> ProtocolIdAt(protocols, i) == Bits(ArrayElement(protocols, i / 4), (i % 4) * 8 + 7, (i % 4) * 8))
        && (forall|i: UInt32| i < num_protocols ==> ProtocolIdAt(protocols, i) != BASE_PROTOCOL_ID)
        && (forall|i: UInt32| i + 1 < num_protocols ==> ProtocolIdAt(protocols, i) < ProtocolIdAt(protocols, i + 1))
    ))
    && (old_s == new_s)
}