pub open spec fn ffa_partition_info_get_regs_spec(uuid_lo: UInt64, uuid_hi: UInt64, start_index: UInt16, tag: UInt16, result: UInt32, error_code: Int32, last_index: UInt16, current_index: UInt16, info_tag: UInt16, desc_size: UInt16, partition_info: [UInt64; 15], old_s: S, new_s: S) -> bool {
  (!IsValidUuid(old_s, uuid_lo, uuid_hi) ==> ResultEqual(error_code, INVALID_PARAMETERS))
  && (!IsValidStartIndex(old_s, uuid_lo, uuid_hi, start_index) ==> ResultEqual(error_code, INVALID_PARAMETERS))
  && (!IsImplementedAtInstance(old_s, FFA_PARTITION_INFO_GET_REGS, ffa_instance) ==> ResultEqual(error_code, NOT_SUPPORTED))
  && (!CalleeInStateToHandleRequest(old_s) ==> ResultEqual(error_code, DENIED))
  && (start_index > 0 && tag != CalleeInfoTag(old_s, uuid_lo, uuid_hi) ==> ResultEqual(error_code, RETRY))
  && (!CalleeReadyToHandleRequest(old_s) ==> ResultEqual(error_code, NOT_READY))
  && (ResultEqual(result, FFA_SUCCESS64) ==> last_index + 1 == PartitionInfoCount(new_s, uuid_lo, uuid_hi))
  && (ResultEqual(result, FFA_SUCCESS64) ==> last_index >= start_index)
  && (ResultEqual(result, FFA_SUCCESS64) ==> current_index >= start_index)
  && (ResultEqual(result, FFA_SUCCESS64) ==> current_index <= last_index)
  && (ResultEqual(result, FFA_SUCCESS64) ==> info_tag == CalleeInfoTag(new_s, uuid_lo, uuid_hi))
  && (ResultEqual(result, FFA_SUCCESS64) ==> desc_size == 48)
  && (ResultEqual(result, FFA_SUCCESS64) ==> (forall i: UInt64 (start_index <= i && i <= current_index) ==> DescriptorAt(new_s, partition_info, i - start_index) == PartitionInfoEntry(new_s, uuid_lo, uuid_hi, i)))
  && (ResultEqual(result, FFA_SUCCESS64) ==> (!IsNilUuid(old_s, uuid_lo, uuid_hi) ==> (forall n: UInt64 IsReturnedDescriptorBase(n, start_index, current_index) ==> (Reg(new_s, n + 1) == 0 && Reg(new_s, n + 2) == 0))))
  && (ResultEqual(result, FFA_SUCCESS64) ==> (forall n: UInt64 (3 <= n && n <= 17 && !IsReturnedDescriptorRegister(n, start_index, current_index)) ==> Reg(new_s, n) == 0))
  && ((IsValidUuid(old_s, uuid_lo, uuid_hi) &&
       IsValidStartIndex(old_s, uuid_lo, uuid_hi, start_index) &&
       IsImplementedAtInstance(old_s, FFA_PARTITION_INFO_GET_REGS, ffa_instance) &&
       CalleeInStateToHandleRequest(old_s) &&
       !(start_index > 0 && tag != CalleeInfoTag(old_s, uuid_lo, uuid_hi)) &&
       CalleeReadyToHandleRequest(old_s))
    ==> result == FFA_SUCCESS64)
  && (result != FFA_SUCCESS64
    ==> last_index == 0)
  && (result != FFA_SUCCESS64
    ==> current_index == 0)
  && (result != FFA_SUCCESS64
    ==> info_tag == 0)
  && (result != FFA_SUCCESS64
    ==> desc_size == 0)
  && (result != FFA_SUCCESS64
    ==> forall i: UInt64 (start_index <= i && i <= current_index) ==> DescriptorAt(new_s, partition_info, i - start_index) == PartitionInfoEntry(new_s, uuid_lo, uuid_hi, i))
  && (result != FFA_SUCCESS64
    ==> forall n: UInt64 (3 <= n && n <= 17 && !IsReturnedDescriptorRegister(n, start_index, current_index)) ==> Reg(new_s, n) == 0)
}