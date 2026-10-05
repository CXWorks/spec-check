pub open spec fn ffa_partition_info_get_spec(result: Int32, flags: UInt32, uuid: UInt128, caller: Caller, old_s: S, new_s: S) -> bool {
    (!RxBufferIsFree(caller) ==> ResultEqual(result, BUSY))
    && (!RxBufferIsMapped(caller) ==> ResultEqual(result, BUSY))
    && (!IsValidUuid(uuid) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (flags & 1 == 0 ==> (!ResultsFitInRxBuffer(caller, uuid) ==> ResultEqual(result, NO_MEMORY)))
    && (!CalleeCanHandleRequest() ==> ResultEqual(result, DENIED))
    && (!IsImplementedAtInstance(FFA_PARTITION_INFO_GET) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!CalleeIsReady() ==> ResultEqual(result, NOT_READY))
    && (result == FFA_SUCCESS ==> count >= 1)
    && (flags & 1 == 0 ==> (count > 0 ==> RxBuffer(caller) contains count partition information descriptors corresponding to uuid, each of size size))
    && (flags & 1 == 1 ==> count == PartitionCount(uuid))
    && (flags & 1 == 1 ==> size == 0)
}