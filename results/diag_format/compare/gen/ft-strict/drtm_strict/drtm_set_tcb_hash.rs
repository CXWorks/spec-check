pub open spec fn drtm_set_tcb_hash_spec(tcb_hash_table: Address, result: Result<(), RmiStatusCode>, supplemental: Int64, old_s: S, new_s: S) -> bool {
  (!DrtmIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidTcbHashTableAddress(old_s, tcb_hash_table) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidTcbHashTableHeader(old_s, tcb_hash_table) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (exists|i: UInt64| i < TcbHashTableEntryCount(old_s, tcb_hash_table) && !IsValidTcbHashTableEntry(old_s, tcb_hash_table, i) ==> ResultEqual(result, INVALID_DATA) && supplemental == FirstInvalidTcbHashTableEntry(old_s, tcb_hash_table))
  && (ExceedsMaxTcbHashes(old_s, tcb_hash_table) ==> ResultEqual(result, OUT_OF_RESOURCE))
  && (TcbHashesLocked(old_s) ==> ResultEqual(result, DENIED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> TcbHashesRecorded(new_s, tcb_hash_table))
  && (result == SUCCESS ==> SourceOfEntryIgnored(new_s, tcb_hash_table))
  && (result == SUCCESS ==> supplemental == NumValidTcbHashEntries(new_s))
  && ((DrtmIsSupported(old_s) &&
       IsValidTcbHashTableAddress(old_s, tcb_hash_table) &&
       IsValidTcbHashTableHeader(old_s, tcb_hash_table) &&
       !(exists|i: UInt64| i < TcbHashTableEntryCount(old_s, tcb_hash_table) && !IsValidTcbHashTableEntry(old_s, tcb_hash_table, i)) &&
       !ExceedsMaxTcbHashes(old_s, tcb_hash_table) &&
       !TcbHashesLocked(old_s))
    ==> result == SUCCESS)
  && (result != SUCCESS
    ==> TcbHashesRecorded(new_s, tcb_hash_table))
  && (result != SUCCESS
    ==> SourceOfEntryIgnored(new_s, tcb_hash_table))
  && (result != SUCCESS
    ==> supplemental == NumValidTcbHashEntries(new_s))
}