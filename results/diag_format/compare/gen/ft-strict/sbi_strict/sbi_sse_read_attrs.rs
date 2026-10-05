pub open spec fn sbi_sse_read_attrs_spec(event_id: UInt32, base_attr_id: UInt32, attr_count: UInt32, output_phys_lo: Address, output_phys_hi: Address, error: SbiErrorCode, old_s: S, new_s: S) -> bool {
  (!IsReservedEventId(old_s, event_id) && IsValidEventId(old_s, event_id) && !PlatformSupportsEvent(old_s, event_id) ==> ResultEqual(error, SBI_ERR_NOT_SUPPORTED))
  && (!IsValidEventId(old_s, event_id) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (attr_count == 0 ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (exists|i: UInt32| i < attr_count && IsReservedEventAttrId(old_s, base_attr_id + i) ==> ResultEqual(error, SBI_ERR_BAD_RANGE))
  && ((output_phys_lo % (XLEN / 8) != 0) || !SatisfiesSharedMemoryRequirements(old_s, output_phys_lo, output_phys_hi, (XLEN / 8) * attr_count) ==> ResultEqual(error, SBI_ERR_INVALID_ADDRESS))
  && (EventAttrReadFailed(old_s, event_id, base_attr_id, attr_count) ==> ResultEqual(error, SBI_ERR_FAILED))
  && (ResultEqual(error, SBI_SUCCESS) ==> ResultEqual(error, SBI_SUCCESS))
  && (ResultEqual(error, SBI_SUCCESS) ==> forall|i: UInt32| i < attr_count ==> SharedMemoryWordAt(new_s, output_phys_lo, output_phys_hi, (XLEN / 8) * (base_attr_id + i)) == EventAttrValue(new_s, event_id, base_attr_id + i))
  && ((!(IsReservedEventId(old_s, event_id) && IsValidEventId(old_s, event_id) && !PlatformSupportsEvent(old_s, event_id)) &&
       IsValidEventId(old_s, event_id) &&
       !(attr_count == 0) &&
       !(exists|i: UInt32| i < attr_count && IsReservedEventAttrId(old_s, base_attr_id + i)) &&
       !((output_phys_lo % (XLEN / 8) != 0) || !SatisfiesSharedMemoryRequirements(old_s, output_phys_lo, output_phys_hi, (XLEN / 8) * attr_count)) &&
       !(EventAttrReadFailed(old_s, event_id, base_attr_id, attr_count)))
    ==> ResultEqual(error, SBI_SUCCESS))
  && (result != SBI_SUCCESS
    ==> SharedMemoryWordAt(new_s, output_phys_lo, output_phys_hi, (XLEN / 8) * (base_attr_id + 0)) == SharedMemoryWordAt(old_s, output_phys_lo, output_phys_hi, (XLEN / 8) * (base_attr_id + 0)))
  && (result != SBI_SUCCESS
    ==> SharedMemoryWordAt(new_s, output_phys_lo, output_phys_hi, (XLEN / 8) * (base_attr_id + 1)) == SharedMemoryWordAt(old_s, output_phys_lo, output_phys_hi, (XLEN / 8) * (base_attr_id + 1)))
}