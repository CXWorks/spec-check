pub open spec fn sbi_sse_write_attrs_spec(result: sbiret, event_id: UInt32, base_attr_id: UInt32, attr_count: UInt32, input_phys_lo: UInt64, input_phys_hi: UInt64, old_s: S, new_s: S) -> bool {
    (!IsValidEventId(event_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (IsValidEventId(event_id) && !IsEventSupportedByPlatform(event_id) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
    && (attr_count == 0 ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (IsValidEventId(event_id) && Exists(i < attr_count : !IsLegalAttrValue(event_id, base_attr_id + i, InputAttrValue(i))) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (IsValidEventId(event_id) && Exists(i < attr_count : IsReadOnlyAttr(event_id, base_attr_id + i)) ==> ResultEqual(result, SBI_ERR_DENIED))
    && (IsValidEventId(event_id) && Exists(i < attr_count : !AttrValueSatisfiesStateRules(event_id, base_attr_id + i, InputAttrValue(i))) ==> ResultEqual(result, SBI_ERR_INVALID_STATE))
    && (Exists(i < attr_count : IsReservedAttrId(base_attr_id + i)) ==> ResultEqual(result, SBI_ERR_BAD_RANGE))
    && (!IsValidSharedMemory(input_phys_lo, input_phys_hi, (XLEN / 8) * attr_count) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
    && (WriteFailedForUnspecifiedReason() ==> ResultEqual(result, SBI_ERR_FAILED))
    && (ResultEqual(result, SBI_SUCCESS) ==> ForAll(i < attr_count : EventAttr(event_id, base_attr_id + i) == InputSharedMemoryAt((XLEN / 8) * (base_attr_id + i))))
    && (IsLocalEvent(event_id) ==> <attr_scope_local_postcondition>)
    && (IsGlobalEvent(event_id) ==> <attr_scope_global_postcondition>)
}