pub open spec fn telemetry_list_shmti__3_12_4_5_spec(index: UInt32, status: Int32, num_SHMTI: UInt16, SHMTI_desc: [UInt32; 5], result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!IsRequestSupported(old_s, 0x3, 0x1B) ==> ResultEqual(result, NOT_SUPPORTED))
  && (NumShmtiAvailable(old_s) == 0 ==> ResultEqual(result, NOT_FOUND))
  && (result == RSI_SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == RSI_SUCCESS ==> (num_SHMTI) == NumDescriptors(SHMTI_desc))
  && (result == RSI_SUCCESS ==> (num_SHMTI) == RemainingShmtiCount(old_s, num_SHMTI))
  && (result == RSI_SUCCESS ==> (forall i: UInt32| i < (num_SHMTI) ==> DescribesShmti(Entry(SHMTI_desc, i), index + i)))
  && (result == RSI_SUCCESS ==> (forall i: UInt32| i < (num_SHMTI) ==> IsInCallerMemoryMap(ShmtiAddress(Word(Entry(SHMTI_desc, i), 2), Word(Entry(SHMTI_desc, i), 1))))))
  && (result == RSI_SUCCESS ==> (forall i: UInt32| i < (num_SHMTI) ==> Word(Entry(SHMTI_desc, i), 4) == 0))
  && ((IsRequestSupported(old_s, 0x3, 0x1B) &&
       !(NumShmtiAvailable(old_s) == 0))
    ==> result == RSI_SUCCESS)
}