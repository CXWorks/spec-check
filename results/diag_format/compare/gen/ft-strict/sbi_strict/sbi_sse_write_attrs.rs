pub open spec fn sbi_sse_write_attrs_spec(event_id: UInt32, base_attr_id: UInt32, attr_count: UInt32, input_phys_lo: UInt, input_phys_hi: UInt, result: SbiError, old_s: S, new_s: S) -> bool {
  (!IsValidEventId(old_s, event_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (IsValidEventId(old_s, event_id) && !IsEventSupportedByPlatform(old_s, event_id) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
  && (attr_count == 0 ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (exists|i: UInt32| i < attr_count && IsReservedAttrId(old_s, base_attr_id + i) ==> ResultEqual(result, SBI_ERR_BAD_RANGE))
  && (!SharedMemoryMeetsRequirements(old_s, input_phys_lo, input_phys_hi, (XLEN / 8) * attr_count) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
  && (IsValidEventId(old_s, event_id) && (exists|i: UInt32| i < attr_count && IsReadOnlyAttr(old_s, event_id, base_attr_id + i)) ==> ResultEqual(result, SBI_ERR_DENIED))
  && (IsValidEventId(old_s, event_id) && (exists|i: UInt32| i < attr_count && !IsLegalAttrValue(old_s, event_id, base_attr_id + i, InputValueAt(old_s, input_phys_lo, input_phys_hi, (XLEN / 8) * (base_attr_id + i)))) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (IsValidEventId(old_s, event_id) && (exists|i: UInt32| i < attr_count && !AttrValueSatisfiesStateRules(old_s, event_id, base_attr_id + i, InputValueAt(old_s, input_phys_lo, input_phys_hi, (XLEN / 8) * (base_attr_id + i)))) ==> ResultEqual(result, SBI_ERR_INVALID_STATE))
  && (WriteFailedForUnspecifiedReason(old_s, event_id, base_attr_id, attr_count) ==> ResultEqual(result, SBI_ERR_FAILED))
  && (result == SBI_SUCCESS ==> IsLocalEvent(old_s, event_id) ==> (forall|i: UInt32| i < attr_count ==> AttrValue(new_s, event_id, CallingHart(), base_attr_id + i) == InputValueAt(new_s, input_phys_lo, input_phys_hi, (XLEN / 8) * (base_attr_id + i))))
  && (result == SBI_SUCCESS ==> IsGlobalEvent(old_s, event_id) ==> (forall|h: Hart| forall|i: UInt32| i < attr_count ==> AttrValue(new_s, event_id, h, base_attr_id + i) == InputValueAt(new_s, input_phys_lo, input_phys_hi, (XLEN / 8) * (base_attr_id + i))))
  && ((IsValidEventId(old_s, event_id) && IsEventSupportedByPlatform(old_s, event_id) && !(attr_count == 0) && !(exists|i: UInt32| i < attr_count && IsReservedAttrId(old_s, base_attr_id + i)) && SharedMemoryMeetsRequirements(old_s, input_phys_lo, input_phys_hi, (XLEN / 8) * attr_count) && !(IsValidEventId(old_s, event_id) && (exists|i: UInt32| i < attr_count && IsReadOnlyAttr(old_s, event_id, base_attr_id + i))) && !(IsValidEventId(old_s, event_id) && (exists|i: UInt32| i < attr_count && !IsLegalAttrValue(old_s, event_id, base_attr_id + i, InputValueAt(old_s, input_phys_lo, input_phys_hi, (XLEN / 8) * (base_attr_id + i))))) && !(IsValidEventId(old_s, event_id) && (exists|i: UInt32| i < attr_count && !AttrValueSatisfiesStateRules(old_s, event_id, base_attr_id + i, InputValueAt(old_s, input_phys_lo, input_phys_hi, (XLEN / 8) * (base_attr_id + i))))) && !(WriteFailedForUnspecifiedReason(old_s, event_id, base_attr_id, attr_count)))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> IsLocalEvent(old_s, event_id) ==> (forall|i: UInt32| i < attr_count ==> AttrValue(new_s, event_id, CallingHart(), base_attr_id + i) == AttrValue(old_s, event_id, CallingHart(), base_attr_id + i)))
  && (result != SBI_SUCCESS
    ==> IsGlobalEvent(old_s, event_id) ==> (forall|h: Hart| forall|i: UInt32| i < attr_count ==> AttrValue(new_s, event_id, h, base_attr_id + i) == AttrValue(old_s, event_id, h, base_attr_id + i)))
}