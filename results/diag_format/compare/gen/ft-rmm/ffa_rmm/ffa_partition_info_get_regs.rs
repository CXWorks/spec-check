pub open spec fn ffa_partition_info_get_regs_spec(uuid_lo: UInt64, uuid_hi: UInt64, start_index: UInt16, tag: UInt16, result: Result, last_index: UInt16, current_index: UInt16, callee_tag: UInt16, desc_size: UInt16, partition_info: [PartitionInfoDescriptor; 14], old_s: S, new_s: S) -> bool {
  (!IsValidUuid(old_s, uuid_lo, uuid_hi) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidStartIndex(old_s, uuid_lo, uuid_hi, start_index) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsImplementedAtInstance(old_s, FFA_PARTITION_INFO_GET_REGS) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!CalleeCanHandleRequest(old_s) ==> ResultEqual(result, DENIED))
  && (start_index > 0 && tag != CalleeInfoTag(old_s, uuid_lo, uuid_hi) ==> ResultEqual(result, RETRY))
  && (!CalleeIsReady(old_s) ==> ResultEqual(result, NOT_READY))
  && (result == FFA_SUCCESS64 ==> last_index >= start_index)
  && (result == FFA_SUCCESS64 ==> current_index >= start_index)
  && (result == FFA_SUCCESS64 ==> NumEntriesReturned() == (current_index - start_index) + 1)
  && (result == FFA_SUCCESS64 ==> callee_tag == CalleeInfoTag(new_s, uuid_lo, uuid_hi))
  && (result == FFA_SUCCESS64 ==> !IsNilUuid(new_s, uuid_lo, uuid_hi) ==> DescProtocolUuidFieldsAreZero(new_s, partition_info))
  && (result == FFA_SUCCESS64 ==> UnusedRegistersAreZero(new_s, partition_info))
  && (result == FFA_SUCCESS64 ==> (last_index == current_index) ==> AllEntriesReturned(new_s))
  && ((IsValidUuid(old_s, uuid_lo, uuid_hi) &&
       IsValidStartIndex(old_s, uuid_lo, uuid_hi, start_index) &&
       IsImplementedAtInstance(old_s, FFA_PARTITION_INFO_GET_REGS) &&
       CalleeCanHandleRequest(old_s) &&
       !(start_index > 0 && tag != CalleeInfoTag(old_s, uuid_lo, uuid_hi)) &&
       CalleeIsReady(old_s))
    ==> result == FFA_SUCCESS64)
  && (result != FFA_SUCCESS64
    ==> last_index == 0)
  && (result != FFA_SUCCESS64
    ==> current_index == 0)
  && (result != FFA_SUCCESS64
    ==> callee_tag == 0)
  && (result != FFA_SUCCESS64
    ==> desc_size == 0)
  && (result != FFA_SUCCESS64
    ==> partition_info[0] == PartitionInfoDescriptor { desc_protocol_uuid: Uuid { time_low: 0, time_mid: 0, time_hi_and_version: 0, clock_seq: 0, node: [0, 0, 0, 0, 0] }, partition_id: 0, partition_type: 0, partition_name: [0; 64] })
  && (result != FFA_SUCCESS64
    ==> partition_info[1] == PartitionInfoDescriptor { desc_protocol_uuid: Uuid { time_low: 0, time_mid: 0, time_hi_and_version: 0, clock_seq: 0, node: [0, 0, 0, 0, 0] }, partition_id: 0, partition_type: 0, partition_name: [0; 64] })
  && (result != FFA_SUCCESS64
    ==> partition_info[2] == PartitionInfoDescriptor { desc_protocol_uuid: Uuid { time_low: 0, time_mid: 0, time_hi_and_version: 0, clock_seq: 0, node: [0, 0, 0, 0, 0] }, partition_id: 0, partition_type: 0, partition_name: [0; 64] })
  && (result != FFA_SUCCESS64
    ==> partition_info[3] == PartitionInfoDescriptor { desc_protocol_uuid: Uuid { time_low: 0, time_mid: 0, time_hi_and_version: 0, clock_seq: 0, node: [0, 0, 0, 0, 0] }, partition_id: 0, partition_type: 0, partition_name: [0; 64] })
  && (result != FFA_SUCCESS64
    ==> partition_info[4] == PartitionInfoDescriptor { desc_protocol_uuid: Uuid { time_low: 0, time_mid: 0, time_hi_and_version: 0, clock_seq: 0, node: [0, 0, 0, 0, 0] }, partition_id: 0, partition_type: 0, partition_name: [0; 64] })
  && (result != FFA_SUCCESS64
    ==> partition_info[5] == PartitionInfoDescriptor { desc_protocol_uuid: Uuid { time_low: 0, time_mid: 0, time_hi_and_version: 0, clock_seq: 0, node: [0, 0, 0, 0, 0] }, partition_id: 0, partition_type: 0, partition_name: [0; 64] })
  && (result != FFA_SUCCESS64
    ==> partition_info[6] == PartitionInfoDescriptor { desc_protocol_uuid: Uuid { time_low: 0, time_mid: 0, time_hi_and_version: 0, clock_seq: 0, node: [0, 0, 0, 0, 0] }, partition_id: 0, partition_type: 0, partition_name: [0; 64] })
  && (result != FFA_SUCCESS64
    ==> partition_info[7] == PartitionInfoDescriptor { desc_protocol_uuid: Uuid { time_low: 0, time_mid: 0, time_hi_and_version: 0, clock_seq: 0, node: [0, 0, 0, 0, 0] }, partition_id: 0, partition_type: 0, partition_name: [0; 64] })
  && (result != FFA_SUCCESS64
    ==> partition_info[8] == PartitionInfoDescriptor { desc_protocol_uuid: Uuid { time_low: 0, time_mid: 0, time_hi_and_version: 0, clock_seq: 0, node: [0, 0, 0, 0, 0] }, partition_id: 0, partition_type: 0, partition_name: [0; 64] })
  && (result != FFA_SUCCESS64
    ==> partition_info[9] == PartitionInfoDescriptor { desc_protocol_uuid: Uuid { time_low: 0, time_mid: 0, time_hi_and_version: 0, clock_seq: 0, node: [0, 0, 0, 0, 0] }, partition_id: 0, partition_type: 0, partition_name: [0; 64] })
  && (result != FFA_SUCCESS64
    ==> partition_info[10] == PartitionInfoDescriptor { desc_protocol_uuid: Uuid { time_low: 0, time_mid: 0, time_hi_and_version: 0, clock_seq: 0, node: [0, 0, 0, 0, 0] }, partition_id: 0, partition_type: 0, partition_name: [0; 64] })
  && (result != FFA_SUCCESS64
    ==> partition_info[11] == PartitionInfoDescriptor { desc_protocol_uuid: Uuid { time_low: 0, time_mid: 0, time_hi_and_version: 0, clock_seq: 0, node: [0, 0, 0, 0, 0] }, partition_id: 0, partition_type: 0, partition_name: [0; 64] })
  && (result != FFA_SUCCESS64
    ==> partition_info[12] == PartitionInfoDescriptor { desc_protocol_uuid: Uuid { time_low: 0, time_mid: 0, time_hi_and_version: 0, clock_seq: 0, node: [0, 0, 0, 0, 0] }, partition_id: 0, partition_type: 0, partition_name: [0; 64] })
  && (result != FFA_SUCCESS64
    ==> partition_info[13] == PartitionInfoDescriptor { desc_protocol_uuid: Uuid { time_low: 0, time_mid: 0, time_hi_and_version: 0, clock_seq: 0, node: [0, 0, 0, 0, 0] }, partition_id: 0, partition_type: 0, partition_name: [0; 64] })
}