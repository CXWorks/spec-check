pub open spec fn drtm_set_tcb_hash_spec(result: Int64, supplemental: Int64, old_s: S, new_s: S) -> bool {
    (!DrtmIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidTcbHashTableAddress(old_s, tcb_hash_table) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsValidTcbHashTableHeader(old_s, tcb_hash_table) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!AllTcbHashTableEntriesValid(old_s, tcb_hash_table) ==> ResultEqual(result, INVALID_DATA))
    && (RecordedTcbHashCount(old_s) + TcbHashTableEntryCount(tcb_hash_table) > MaxTcbHashes() ==> ResultEqual(result, OUT_OF_RESOURCE))
    && (TcbHashesLocked(old_s) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> ResultEqual(result, SUCCESS))
    && (ResultEqual(result, SUCCESS) ==> RecordedTcbHashes(new_s) == Concat(old_s(RecordedTcbHashes()), TcbHashTableDigests(tcb_hash_table)))
    && (ResultEqual(result, SUCCESS) ==> ValidTcbHashTableEntryCount(tcb_hash_table) == supplemental)
}