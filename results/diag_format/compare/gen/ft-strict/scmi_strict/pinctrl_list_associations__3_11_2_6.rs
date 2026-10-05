pub open spec fn pinctrl_list_associations__3_11_2_6_spec(identifier: UInt32, flags: UInt32, index: UInt32, result: Result<Int32, (Int32, (UInt32, UInt32, [UInt16; 1]))>, out_flags: UInt32, array: [UInt16; 1], old_s: S, new_s: S) -> bool {
  (!GroupOrFunctionExists(old_s, identifier, (flags & 3) as int) ==> ResultEqual(result, NOT_FOUND))
  && (!IsRequestSupported(old_s, identifier, flags, index) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!AgentMayListAssociations(old_s, CallingAgent(old_s), identifier, (flags & 3) as int) ==> ResultEqual(result, DENIED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> (out_flags & 2047) == ArrayLength(array))
  && (result == SUCCESS ==> (out_flags & 65280) == AssociationCount(old_s, identifier, (flags & 3) as int) - index - (out_flags & 2047))
  && (result == SUCCESS ==> (out_flags & 61440) == 0)
  && (result == SUCCESS ==> (forall i: UInt32, i < (out_flags & 2047) ==> ArrayAt(array, i) == AssociationAt(old_s, identifier, (flags & 3) as int, index + i)))
  && (result == SUCCESS ==> (forall i: UInt32, i + 1 < (out_flags & 2047) ==> ArrayAt(array, i) < ArrayAt(array, i + 1)))
  && ((!(GroupOrFunctionExists(old_s, identifier, (flags & 3) as int)) &&
       IsRequestSupported(old_s, identifier, flags, index) &&
       AgentMayListAssociations(old_s, CallingAgent(old_s), identifier, (flags & 3) as int))
    ==> ResultEqual(result, SUCCESS))
  && (result != SUCCESS
    ==> (out_flags & 2047) == 0)
  && (result != SUCCESS
    ==> (out_flags & 65280) == AssociationCount(old_s, identifier, (flags & 3) as int) - index - 0)
  && (result != SUCCESS
    ==> (out_flags & 61440) == 0)
  && (!(result == SUCCESS && ((out_flags & 2047) == 0)) ==> (forall i: UInt32, i < (out_flags & 2047) ==> ArrayAt(array, i) == AssociationAt(old_s, identifier, (flags & 3) as int, index + i)))
}