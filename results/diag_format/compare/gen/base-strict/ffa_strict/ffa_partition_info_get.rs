pub open spec fn ffa_partition_info_get_spec(result: FfaReturnStatus, count: UInt32, size: UInt32, old_s: S, new_s: S) -> bool {
    let caller = old_s.caller;
    let callee = old_s.callee;
    let fid = old_s.cmd_input_fid;
    let uuid = old_s.cmd_input_uuid;
    let flags = old_s.cmd_input_flags;
    let instance = old_s.instance;

    // Failure conditions
    (Bits64(flags as int, 0, 0) == 0 && (!RxBufferFree(caller) || !RxBufferMapped(caller)) ==> ResultEqual(result, BUSY))
    && (!IsValidUuid(uuid) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (Bits64(flags as int, 0, 0) == 0 && !PartitionInfoFitsInRxBuffer(caller, uuid) ==> ResultEqual(result, NO_MEMORY))
    && (!CalleeInStateToHandleRequest(callee) ==> ResultEqual(result, DENIED))
    && (!IsImplementedAtInstance(FFA_PARTITION_INFO_GET, instance) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!CalleeReadyToHandleRequest(callee) ==> ResultEqual(result, NOT_READY))

    // Success conditions
    && (ResultEqual(result, FFA_SUCCESS) ==> (count >= 1))
    && (ResultEqual(result, FFA_SUCCESS) ==> (Bits64(flags as int, 0, 0) == 0 ==> count == PartitionInfoDescriptorCount(uuid)))
    && (ResultEqual(result, FFA_SUCCESS) ==> (Bits64(flags as int, 0, 0) == 0 ==> RxBufferHoldsPartitionInfoDescriptors(caller, uuid, count, size)))
    && (ResultEqual(result, FFA_SUCCESS) ==> (Bits64(flags as int, 0, 0) == 0 ==> size == PartitionInfoDescriptorSize(uuid)))
    && (ResultEqual(result, FFA_SUCCESS) ==> (Bits64(flags as int, 0, 0) == 1 ==> count == DeployedPartitionCount(uuid)))
    && (ResultEqual(result, FFA_SUCCESS) ==> (Bits64(flags as int, 0, 0) == 1 ==> size == 0))
}