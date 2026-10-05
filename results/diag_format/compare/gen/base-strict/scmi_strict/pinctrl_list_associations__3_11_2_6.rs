pub open spec fn pinctrl_list_associations__3_11_2_6_spec(result: Int32, out_flags: UInt32, array: Array<UInt16>, old_s: S, new_s: S) -> bool {
    (!GroupOrFunctionExists(identifier, Bits64(out_flags, 1, 0) as int) ==> ResultEqual(result, NOT_FOUND))
    && (!IsRequestSupported(identifier, flags, index) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentMayListAssociations(CallingAgent(), identifier, Bits64(out_flags, 1, 0) as int) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (Bits64(out_flags, 11, 0) as int == ArrayLength(array) as int))
    && (ResultEqual(result, SUCCESS) ==> (Bits64(out_flags, 31, 16) as int == AssociationCount(identifier, Bits64(out_flags, 1, 0) as int) as int - index as int - Bits64(out_flags, 11, 0) as int))
    && (ResultEqual(result, SUCCESS) ==> (Bits64(out_flags, 15, 12) as int == 0))
    && (ResultEqual(result, SUCCESS) ==> (forall i: UInt32 | i < Bits64(out_flags, 11, 0) as int ==> ArrayAt(array, i) == AssociationAt(identifier, Bits64(out_flags, 1, 0) as int, index as int + i)))
    && (ResultEqual(result, SUCCESS) ==> (forall i: UInt32 | i + 1 < Bits64(out_flags, 11, 0) as int ==> ArrayAt(array, i) < ArrayAt(array, i + 1)))
    && (old_s == new_s)
}