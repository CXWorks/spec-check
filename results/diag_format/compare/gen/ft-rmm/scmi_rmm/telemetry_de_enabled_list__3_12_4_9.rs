pub open spec fn telemetry_de_enabled_list__3_12_4_9_spec(index: UInt32, flags: UInt32, selector: UInt1, status: Int32, num_remaining: UInt16, num_returned: UInt16, array: [EnabledListEntry; 16], old_s: S, new_s: S) -> bool {
  (!IsValidEnabledListIndex(old_s, index, selector) ==> ResultEqual(status, OUT_OF_RANGE))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> Length(array) == num_returned)
  && (ResultEqual(status, SUCCESS) ==> (selector == 0 && AllEntriesAreEnabledDes(array)) || (selector == 1 && AllEntriesAreEnabledEventGroups(array)))
  && (ResultEqual(status, SUCCESS) ==> ForAll(array, e => e.mode == 1 || e.mode == 2))
  && ((IsValidEnabledListIndex(old_s, index, selector))
    ==> ResultEqual(status, SUCCESS))
}