pub open spec fn sbi_sse_read_attrs_spec(event_id: UInt32, base_attr_id: UInt32, attr_count: UInt32, output_phys_lo: Address, output_phys_hi: Address, result: SbiErrorCode, old_s: S, new_s: S) -> bool {
  (IsValidEventId(old_s, event_id) && !IsReservedEventId(old_s, event_id) && !PlatformSupportsEvent(old_s, event_id) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
  && (!IsValidEventId(old_s, event_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (attr_count == 0 ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (Exists(i, 0 <= i < attr_count, IsReservedAttrId(old_s, base_attr_id + i)) ==> ResultEqual(result, SBI_ERR_BAD_RANGE))
  && (!SatisfiesSharedMemoryRequirements(old_s, output_phys_lo, output_phys_hi, (XLEN / 8) * attr_count) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
  && (ReadFailedForOtherReason(old_s) ==> ResultEqual(result, SBI_ERR_FAILED))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> ForAll(i, 0 <= i < attr_count, SharedMemoryAt(new_s, OutputPhysAddr(output_phys_lo, output_phys_hi) + (XLEN / 8) * (base_attr_id + i)) == EventAttr(new_s, event_id, base_attr_id + i)))
  && ((! (IsValidEventId(old_s, event_id) && !IsReservedEventId(old_s, event_id) && !PlatformSupportsEvent(old_s, event_id)) &&
       IsValidEventId(old_s, event_id) &&
       !(attr_count == 0) &&
       !(Exists(i, 0 <= i < attr_count, IsReservedAttrId(old_s, base_attr_id + i))) &&
       SatisfiesSharedMemoryRequirements(old_s, output_phys_lo, output_phys_hi, (XLEN / 8) * attr_count) &&
       !(ReadFailedForOtherReason(old_s)))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> ForAll(i, 0 <= i < attr_count, SharedMemoryAt(new_s, OutputPhysAddr(output_phys_lo, output_phys_hi) + (XLEN / 8) * (base_attr_id + i)) == SharedMemoryAt(old_s, OutputPhysAddr(output_phys_lo, output_phys_hi) + (XLEN / 8) * (base_attr_id + i))))
}