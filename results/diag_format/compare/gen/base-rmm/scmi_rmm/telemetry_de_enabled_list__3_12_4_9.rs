pub open spec fn telemetry_de_enabled_list__3_12_4_9_spec(result: Int32, num_remaining: UInt16, num_returned: UInt16, array: [UInt32], old_s: S, new_s: S) -> bool {
    (!IsValidEnabledListIndex(index, selector) ==> ResultEqual(result, OUT_OF_RANGE))
    && (ResultEqual(result, SUCCESS) ==> (Length(array) == num_returned))
    && (ResultEqual(result, SUCCESS) ==> ((selector == 0 && AllEntriesAreEnabledDes(array)) || (selector == 1 && AllEntriesAreEnabledEventGroups(array))))
    && (ResultEqual(result, SUCCESS) ==> ForAll(array, e => e.mode == 1 || e.mode == 2))
    && (old_s == new_s)
}