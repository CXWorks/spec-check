pub open spec fn ffa_partition_info_get_spec(uuid: UInt128, flags: UInt32, result: Result<(), Int32>, count: UInt32, size: UInt32, old_s: S, new_s: S) -> bool {
  ((flags & 1) == 0 && (!RxBufferIsFree(old_s, caller) || !RxBufferIsMapped(old_s, caller)) ==> ResultEqual(result, BUSY))
  && (!IsValidUuid(old_s, uuid) ==> ResultEqual(result, INVALID_PARAMETERS))
  && ((flags & 1) == 0 && !ResultsFitInRxBuffer(old_s, caller, uuid) ==> ResultEqual(result, NO_MEMORY))
  && (!CalleeCanHandleRequest(old_s) ==> ResultEqual(result, DENIED))
  && (!IsImplementedAtInstance(old_s, FFA_PARTITION_INFO_GET) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!CalleeIsReady(old_s) ==> ResultEqual(result, NOT_READY))
  && (result == FFA_SUCCESS ==> ReturnedFunction(new_s) == FFA_SUCCESS)
  && (result == FFA_SUCCESS ==> count >= 1)
  && (result == FFA_SUCCESS && (flags & 1) == 0 ==> RxBuffer(new_s, caller) contains count partition information descriptors corresponding to uuid, each of size size)
  && (result == FFA_SUCCESS && (flags & 1) == 1 ==> count == PartitionCount(new_s, uuid))
  && (result == FFA_SUCCESS && (flags & 1) == 1 ==> size == 0)
  && ((!( (flags & 1) == 0 && (!RxBufferIsFree(old_s, caller) || !RxBufferIsMapped(old_s, caller))) &&
       IsValidUuid(old_s, uuid) &&
       !((flags & 1) == 0 && !ResultsFitInRxBuffer(old_s, caller, uuid)) &&
       CalleeCanHandleRequest(old_s) &&
       IsImplementedAtInstance(old_s, FFA_PARTITION_INFO_GET) &&
       CalleeIsReady(old_s))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> ReturnedFunction(new_s) == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> count == 0)
  && (result != FFA_SUCCESS
    ==> size == 0)
}