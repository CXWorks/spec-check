pub open spec fn sbi_sse_write_attrs_spec(result: SbiError, event_id: UInt32, base_attr_id: UInt32, attr_count: UInt32, input_phys_lo: UInt, input_phys_hi: UInt, old_s: S, new_s: S) -> bool {
    (!IsValidEventId(event_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (IsValidEventId(event_id) && !IsEventSupportedByPlatform(event_id) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
    && (attr_count == 0 ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (exists|i: UInt32| i < attr_count && IsReservedAttrId(base_attr_id + i) ==> ResultEqual(result, SBI_ERR_BAD_RANGE))
    && (!SharedMemoryMeetsRequirements(input_phys_lo, input_phys_hi, (UInt as int) * attr_count) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
    && (IsValidEventId(event_id) && (exists|i: UInt32| i < attr_count && IsReadOnlyAttr(event_id, base_attr_id + i)) ==> ResultEqual(result, SBI_ERR_DENIED))
    && (IsValidEventId(event_id) && (exists|i: UInt32| i < attr_count && !IsLegalAttrValue(event_id, base_attr_id + i, InputValueAt(input_phys_lo, input_phys_hi, (UInt as int) * (base_attr_id + i)))) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (IsValidEventId(event_id) && (exists|i: UInt32| i < attr_count && !AttrValueSatisfiesStateRules(event_id, base_attr_id + i, InputValueAt(input_phys_lo, input_phys_hi, (UInt as int) * (base_attr_id + i)))) ==> ResultEqual(result, SBI_ERR_INVALID_STATE))
    && (WriteFailedForUnspecifiedReason(event_id, base_attr_id, attr_count) ==> ResultEqual(result, SBI_ERR_FAILED))
    && (ResultEqual(result, SBI_SUCCESS) ==> (IsLocalEvent(event_id) ==> (forall|i: UInt32| i < attr_count ==> AttrValue(event_id, CallingHart(), base_attr_id + i) == InputValueAt(input_phys_lo, input_phys_hi, (UInt as int) * (base_attr_id + i)))) && (IsGlobalEvent(event_id) ==> (forall|h: Hart| forall|i: UInt32| i < attr_count ==> AttrValue(event_id, h, base_attr_id + i) == InputValueAt(input_phys_lo, input_phys_hi, (UInt as int) * (base_attr_id + i)))))
}