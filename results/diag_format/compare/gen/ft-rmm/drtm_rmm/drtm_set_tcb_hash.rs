pub open spec fn drtm_set_tcb_hash_spec(tcb_hash_table: Address, result: Result<(), DrtmStatusCode>, supplemental: Int64, old_s: S, new_s: S) -> bool {
  (!DrtmIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidTcbHashTableAddress(old_s, tcb_hash_table) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidTcbHashTableHeader(old_s, tcb_hash_table) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!AllTcbHashTableEntriesValid(old_s, tcb_hash_table) ==> ResultEqual(result, INVALID_DATA))
  && (RecordedTcbHashCount(old_s) + TcbHashTableEntryCount(old_s, tcb_hash_table) > MaxTcbHashes(old_s) ==> ResultEqual(result, OUT_OF_RESOURCE))
  && (TcbHashesLocked(old_s) ==> ResultEqual(result, DENIED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> RecordedTcbHashes(new_s) == Concat(old_s(RecordedTcbHashes()), TcbHashTableDigests(new_s, tcb_hash_table)))
  && (result == SUCCESS ==> supplemental == ValidTcbHashTableEntryCount(new_s, tcb_hash_table))
  && ((DrtmIsSupported(old_s) &&
       IsValidTcbHashTableAddress(old_s, tcb_hash_table) &&
       IsValidTcbHashTableHeader(old_s, tcb_hash_table) &&
       AllTcbHashTableEntriesValid(old_s, tcb_hash_table) &&
       !(RecordedTcbHashCount(old_s) + TcbHashTableEntryCount(old_s, tcb_hash_table) > MaxTcbHashes(old_s)) &&
       !TcbHashesLocked(old_s))
    ==> ResultEqual(result, SUCCESS))
  && (result != SUCCESS
    ==> RecordedTcbHashes(new_s) == RecordedTcbHashes(old_s))
}