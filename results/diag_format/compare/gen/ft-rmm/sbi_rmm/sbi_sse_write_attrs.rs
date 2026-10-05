pub open spec fn sbi_sse_write_attrs_spec(event_id: UInt32, base_attr_id: UInt32, attr_count: UInt32, input_phys_lo: unsigned long, input_phys_hi: unsigned long, result: sbiret, old_s: S, new_s: S) -> bool {
  (IsValidEventId(old_s, event_id) && !IsEventSupportedByPlatform(old_s, event_id) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
  && (!IsValidEventId(old_s, event_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (attr_count == 0 ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (IsValidEventId(old_s, event_id) && Exists(i < attr_count : !IsLegalAttrValue(old_s, event_id, base_attr_id + i, InputAttrValue(i))) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (IsValidEventId(old_s, event_id) && Exists(i < attr_count : IsReadOnlyAttr(old_s, event_id, base_attr_id + i)) ==> ResultEqual(result, SBI_ERR_DENIED))
  && (IsValidEventId(old_s, event_id) && Exists(i < attr_count : !AttrValueSatisfiesStateRules(old_s, event_id, base_attr_id + i, InputAttrValue(i))) ==> ResultEqual(result, SBI_ERR_INVALID_STATE))
  && (Exists(i < attr_count : IsReservedAttrId(old_s, base_attr_id + i)) ==> ResultEqual(result, SBI_ERR_BAD_RANGE))
  && (!IsValidSharedMemory(old_s, input_phys_lo, input_phys_hi, (XLEN / 8) * attr_count) ==> ResultEqual(result, SBI_ERR_INVALID_ADDRESS))
  && (WriteFailedForUnspecifiedReason() ==> ResultEqual(result, SBI_ERR_FAILED))
  && (ResultEqual(result, SBI_SUCCESS) ==> ResultEqual(result, SBI_SUCCESS))
  && (ResultEqual(result, SBI_SUCCESS) ==> ForAll(i < attr_count : EventAttr(new_s, event_id, base_attr_id + i) == InputSharedMemoryAt((XLEN / 8) * (base_attr_id + i))))
  && (ResultEqual(result, SBI_SUCCESS) && IsLocalEvent(old_s, event_id) ==> true)
  && (ResultEqual(result, SBI_SUCCESS) && IsGlobalEvent(old_s, event_id) ==> true)
  && ((!(IsValidEventId(old_s, event_id) && !IsEventSupportedByPlatform(old_s, event_id)) &&
       IsValidEventId(old_s, event_id) &&
       !(attr_count == 0) &&
       !(IsValidEventId(old_s, event_id) && Exists(i < attr_count : !IsLegalAttrValue(old_s, event_id, base_attr_id + i, InputAttrValue(i)))) &&
       !(IsValidEventId(old_s, event_id) && Exists(i < attr_count : IsReadOnlyAttr(old_s, event_id, base_attr_id + i))) &&
       !(IsValidEventId(old_s, event_id) && Exists(i < attr_count : !AttrValueSatisfiesStateRules(old_s, event_id, base_attr_id + i, InputAttrValue(i)))) &&
       !(Exists(i < attr_count : IsReservedAttrId(old_s, base_attr_id + i))) &&
       IsValidSharedMemory(old_s, input_phys_lo, input_phys_hi, (XLEN / 8) * attr_count) &&
       !(WriteFailedForUnspecifiedReason()))
    ==> ResultEqual(result, SBI_SUCCESS))
  && (result != SBI_SUCCESS
    ==> ForAll(i < attr_count : EventAttr(new_s, event_id, base_attr_id + i) == EventAttr(old_s, event_id, base_attr_id + i)))
}