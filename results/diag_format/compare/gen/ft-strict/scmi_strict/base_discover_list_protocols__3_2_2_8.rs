pub open spec fn base_discover_list_protocols__3_2_2_8_spec(skip: UInt32, result: Result<Int32, [UInt32; 1]), old_s: S, new_s: S) -> bool {
  (!IsValidSkip(old_s, skip) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> forall i: UInt32| i < result[1] ==> ProtocolIdAt(result[2], i) == AccessibleProtocolAt(old_s, skip + i))
  && (result == SUCCESS ==> forall i: UInt32| i < result[1] ==> ProtocolIdAt(result[2], i) == Bits(ArrayElement(result[2], i / 4), (i % 4) * 8 + 7, (i % 4) * 8))
  && (result == SUCCESS ==> forall i: UInt32| i < result[1] ==> ProtocolIdAt(result[2], i) != BASE_PROTOCOL_ID)
  && (result == SUCCESS ==> forall i: UInt32| i + 1 < result[1] ==> ProtocolIdAt(result[2], i) < ProtocolIdAt(result[2], i + 1))
  && ((IsValidSkip(old_s, skip))
    ==> ResultEqual(result, SUCCESS))
}