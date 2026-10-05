pub open spec fn telemetry_list_shmti__3_12_4_5_spec(index: UInt32, status: Int32, num_SHMTI: [UInt16; 2], SHMTI_desc: [Array([UInt32; 5]); 1], old_s: S, new_s: S) -> bool {
  (!IsTelemetryListShmtiSupported(old_s) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AnyShmtiAvailable(old_s) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> num_SHMTI[1] == NumShmtiDescriptorsReturned(new_s))
  && (ResultEqual(status, SUCCESS) ==> num_SHMTI[0] == NumShmtiRemaining(new_s, index, num_SHMTI[1]))
  && (ResultEqual(status, SUCCESS) ==> SHMTI_desc[0][0] == ShmtiDescriptor(new_s, index))
  && (ResultEqual(status, SUCCESS) ==> IsInCallerMemoryMap((SHMTI_desc[0][2] << 32) | SHMTI_desc[0][1]))
  && (ResultEqual(status, SUCCESS) ==> SHMTI_desc[0][4] == 0)
  && ((IsTelemetryListShmtiSupported(old_s) &&
       AnyShmtiAvailable(old_s))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> num_SHMTI[1] == 0)
  && (result != SUCCESS
    ==> num_SHMTI[0] == 0)
  && (result != SUCCESS
    ==> SHMTI_desc[0][0] == 0)
  && (result != SUCCESS
    ==> SHMTI_desc[0][1] == 0)
  && (result != SUCCESS
    ==> SHMTI_desc[0][2] == 0)
  && (result != SUCCESS
    ==> SHMTI_desc[0][3] == 0)
  && (result != SUCCESS
    ==> SHMTI_desc[0][4] == 0)
}