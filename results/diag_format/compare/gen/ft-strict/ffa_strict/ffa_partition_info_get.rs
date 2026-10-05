pub open spec fn ffa_partition_info_get_spec(uuid: UInt128, flags: UInt32, result: FfaReturnStatus, count: UInt32, size: UInt32, old_s: S, new_s: S) -> bool {
  ((flags & 0x1) == 0 && (!RxBufferFree(old_s, caller) || !RxBufferMapped(old_s, caller)) ==> ResultEqual(result, BUSY))
  && (!IsValidUuid(old_s, uuid) ==> ResultEqual(result, INVALID_PARAMETERS))
  && ((flags & 0x1) == 0 && !PartitionInfoFitsInRxBuffer(old_s, caller, uuid) ==> ResultEqual(result, NO_MEMORY))
  && (!CalleeInStateToHandleRequest(old_s, callee) ==> ResultEqual(result, DENIED))
  && (!IsImplementedAtInstance(old_s, FFA_PARTITION_INFO_GET, instance) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!CalleeReadyToHandleRequest(old_s, callee) ==> ResultEqual(result, NOT_READY))
  && (result == FFA_SUCCESS)
  && (result == FFA_SUCCESS ==> count >= 1)
  && (result == FFA_SUCCESS && (flags & 0x1) == 0 ==> count == PartitionInfoDescriptorCount(old_s, uuid))
  && (result == FFA_SUCCESS && (flags & 0x1) == 0 ==> RxBufferHoldsPartitionInfoDescriptors(old_s, caller, uuid, count, size))
  && (result == FFA_SUCCESS && (flags & 0x1) == 0 ==> size == PartitionInfoDescriptorSize(old_s, uuid))
  && (result == FFA_SUCCESS && (flags & 0x1) == 1 ==> count == DeployedPartitionCount(old_s, uuid))
  && (result == FFA_SUCCESS && (flags & 0x1) == 1 ==> size == 0)
  && ((!( (flags & 0x1) == 0 && (!RxBufferFree(old_s, caller) || !RxBufferMapped(old_s, caller))) &&
       IsValidUuid(old_s, uuid) &&
       !((flags & 0x1) == 0 && !PartitionInfoFitsInRxBuffer(old_s, caller, uuid)) &&
       CalleeInStateToHandleRequest(old_s, callee) &&
       IsImplementedAtInstance(old_s, FFA_PARTITION_INFO_GET, instance) &&
       CalleeReadyToHandleRequest(old_s, callee))
    ==> result == FFA_SUCCESS)
}