pub open spec fn sbi_sse_read_attrs_spec(result: SbiErrorCode, old_s: S, new_s: S, event_id: UInt32, base_attr_id: UInt32, attr_count: UInt32, output_phys_lo: UInt, output_phys_hi: UInt) -> bool {
    (!IsValidEventId(event_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (IsValidEventId(event_id) && !IsReservedEventId(event_id) && !PlatformSupportsEvent(event_id) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
    && (attr_count == 0 ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (Exists(i, 0 <= i < attr_count, IsReservedAttrId(base_attr_id + i)) ==> ResultEqual(result, SBI_ERR_BAD_RANGE))
    && (!SatisfiesSharedMemoryRequirements(output_phys_lo, output_phys_hi, (old_s.XLEN as int / 8) * (attr_count as int)) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
    && (ReadFailedForOtherReason() ==> ResultEqual(result, SBI_ERR_FAILED))
    && (ResultEqual(result, SBI_SUCCESS) ==> ForAll(i, 0 <= i < attr_count, SharedMemoryAt(OutputPhysAddr(output_phys_lo, output_phys_hi) + (old_s.XLEN as int / 8) * ((base_attr_id + i) as int)) == EventAttr(event_id, base_attr_id + i)))
}