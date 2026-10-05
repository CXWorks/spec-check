pub open spec fn drtm_set_tcb_hash_spec(result: Int64, supplemental: Int64, old_s: S, new_s: S) -> bool {
    (!DrtmIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidTcbHashTableAddress(old_s, tcb_hash_table) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsValidTcbHashTableHeader(old_s, tcb_hash_table) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (exists|i: UInt64| i < TcbHashTableEntryCount(tcb_hash_table) && !IsValidTcbHashTableEntry(tcb_hash_table, i) ==> (ResultEqual(result, INVALID_DATA) && supplemental == FirstInvalidTcbHashTableEntry(tcb_hash_table)))
    && (ExceedsMaxTcbHashes(tcb_hash_table) ==> ResultEqual(result, OUT_OF_RESOURCE))
    && (TcbHashesLocked() ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (TcbHashesRecorded(tcb_hash_table) && SourceOfEntryIgnored(tcb_hash_table) && supplemental == NumValidTcbHashEntries()))
}