pub open spec fn telemetry_list_shmti__3_12_4_5_spec(status: Int32, num_SHMTI_low: UInt16, num_SHMTI_high: UInt16, SHMTI_desc: Array<UInt32>, index: UInt32, old_s: S, new_s: S) -> bool {
    (!IsTelemetryListShmtiSupported() ==> ResultEqual(status, NOT_SUPPORTED))
    && (!AnyShmtiAvailable() ==> ResultEqual(status, NOT_FOUND))
    && (ResultEqual(status, SUCCESS) ==> (num_SHMTI_low == NumShmtiDescriptorsReturned() && num_SHMTI_high == NumShmtiRemaining(index, num_SHMTI_low) && SHMTI_desc[0] == ShmtiDescriptor(index) && (forall i: UInt16 | i < num_SHMTI_low ==> (IsInCallerMemoryMap((SHMTI_desc[i * 5 + 2] as UInt64 << 32) | SHMTI_desc[i * 5 + 1]) && SHMTI_desc[i * 5 + 4] == 0))))
}