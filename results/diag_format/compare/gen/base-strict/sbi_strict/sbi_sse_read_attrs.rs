pub open spec fn sbi_sse_read_attrs_spec(error: SbiErrorCode, event_id: UInt32, base_attr_id: UInt32, attr_count: UInt32, output_phys_lo: UInt64, output_phys_hi: UInt64, old_s: S, new_s: S) -> bool {
    (!IsReservedEventId(event_id) && IsValidEventId(event_id) && !PlatformSupportsEvent(event_id) ==> ResultEqual(error, SBI_ERR_NOT_SUPPORTED))
    && (!IsValidEventId(event_id) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (attr_count == 0 ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (exists|i: UInt32| i < attr_count && IsReservedEventAttrId(base_attr_id + i) ==> ResultEqual(error, SBI_ERR_BAD_RANGE))
    && ((output_phys_lo % (XLEN / 8) != 0) || !SatisfiesSharedMemoryRequirements(output_phys_lo, output_phys_hi, (XLEN / 8) * attr_count) ==> ResultEqual(error, SBI_ERR_INVALID_ADDRESS))
    && (EventAttrReadFailed(event_id, base_attr_id, attr_count) ==> ResultEqual(error, SBI_ERR_FAILED))
    && (ResultEqual(error, SBI_SUCCESS) ==> forall|i: UInt32| i < attr_count ==> SharedMemoryWordAt(output_phys_lo, output_phys_hi, (XLEN / 8) * (base_attr_id + i)) == EventAttrValue(event_id, base_attr_id + i))
}