pub open spec fn telemetry_list_shmti_spec(result: Int32, num_SHMTI: UInt16, SHMTI_desc: Array<UInt32>, old_s: S, new_s: S) -> bool {
    (!IsRequestSupported(message_id, protocol_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (NumShmtiAvailable() == 0 ==> ResultEqual(result, NOT_FOUND))
    && ResultEqual(result, SUCCESS)
    && (Bits(num_SHMTI, 15, 0) == NumDescriptors(SHMTI_desc))
    && (Bits(num_SHMTI, 31, 16) == RemainingShmtiCount(index, Bits(num_SHMTI, 15, 0)))
    && (forall|i: UInt32| i < Bits(num_SHMTI, 15, 0) ==> DescribesShmti(Entry(SHMTI_desc, i), index + i))
    && (forall|i: UInt32| i < Bits(num_SHMTI, 15, 0) ==> IsInCallerMemoryMap(ShmtiAddress(Word(Entry(SHMTI_desc, i), 2), Word(Entry(SHMTI_desc, i), 1))))
    && (forall|i: UInt32| i < Bits(num_SHMTI, 15, 0) ==> Word(Entry(SHMTI_desc, i), 4) == 0)
    && (old_s == new_s)
}