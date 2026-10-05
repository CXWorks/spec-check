pub open spec fn sbi_sse_write_attrs_spec(result: SbiRet, old_s: S, new_s: S, event_id: UInt32, base_attr_id: UInt32, attr_count: UInt32, input_phys_lo: UInt64, input_phys_hi: UInt64) -> bool {
    let xb: int = SbiXlenBytes(old_s) as int;
    (!SseEventIdIsValid(old_s, event_id) ==> result.error != SBI_SUCCESS)
    && ((attr_count as int) == 0 ==> result.error != SBI_SUCCESS)
    && ((SseEventIdIsValid(old_s, event_id) && !SseEventIsSupported(old_s, event_id)) ==> result.error != SBI_SUCCESS)
    && ((exists|i: int| 0 <= i < (attr_count as int) && SseAttrIdIsReserved((base_attr_id as int) + i)) ==> result.error != SBI_SUCCESS)
    && (((input_phys_lo as int) % xb != 0 || !SbiSharedMemIsValid(old_s, input_phys_lo, input_phys_hi, xb * (attr_count as int))) ==> result.error != SBI_SUCCESS)
    && ((SseEventIdIsValid(old_s, event_id) && (exists|i: int| 0 <= i < (attr_count as int) && (
            !SseAttrValueIsLegal(old_s, event_id, (base_attr_id as int) + i, SbiSharedMemReadXlen(old_s, input_phys_lo, input_phys_hi, xb * ((base_attr_id as int) + i)))
            || SseAttrIsReadOnly(old_s, event_id, (base_attr_id as int) + i)
            || !SseAttrValueSatisfiesStateRules(old_s, event_id, (base_attr_id as int) + i, SbiSharedMemReadXlen(old_s, input_phys_lo, input_phys_hi, xb * ((base_attr_id as int) + i)))))) ==> result.error != SBI_SUCCESS)
    && (result.error == SBI_ERR_NOT_SUPPORTED ==> (SseEventIdIsValid(old_s, event_id) && !SseEventIsSupported(old_s, event_id)))
    && (result.error == SBI_ERR_INVALID_PARAM ==> (
            !SseEventIdIsValid(old_s, event_id)
            || (attr_count as int) == 0
            || (SseEventIdIsValid(old_s, event_id) && (exists|i: int| 0 <= i < (attr_count as int)
                && !SseAttrValueIsLegal(old_s, event_id, (base_attr_id as int) + i, SbiSharedMemReadXlen(old_s, input_phys_lo, input_phys_hi, xb * ((base_attr_id as int) + i)))
                && (forall|j: int| 0 <= j < i ==> (
                    SseAttrValueIsLegal(old_s, event_id, (base_attr_id as int) + j, SbiSharedMemReadXlen(old_s, input_phys_lo, input_phys_hi, xb * ((base_attr_id as int) + j)))
                    && !SseAttrIsReadOnly(old_s, event_id, (base_attr_id as int) + j)
                    && SseAttrValueSatisfiesStateRules(old_s, event_id, (base_attr_id as int) + j, SbiSharedMemReadXlen(old_s, input_phys_lo, input_phys_hi, xb * ((base_attr_id as int) + j)))))))))
    && (result.error == SBI_ERR_DENIED ==> (SseEventIdIsValid(old_s, event_id) && (exists|i: int| 0 <= i < (attr_count as int)
            && SseAttrIsReadOnly(old_s, event_id, (base_attr_id as int) + i)
            && (forall|j: int| 0 <= j < i ==> (
                SseAttrValueIsLegal(old_s, event_id, (base_attr_id as int) + j, SbiSharedMemReadXlen(old_s, input_phys_lo, input_phys_hi, xb * ((base_attr_id as int) + j)))
                && !SseAttrIsReadOnly(old_s, event_id, (base_attr_id as int) + j)
                && SseAttrValueSatisfiesStateRules(old_s, event_id, (base_attr_id as int) + j, SbiSharedMemReadXlen(old_s, input_phys_lo, input_phys_hi, xb * ((base_attr_id as int) + j))))))))
    && (result.error == SBI_ERR_INVALID_STATE ==> (SseEventIdIsValid(old_s, event_id) && (exists|i: int| 0 <= i < (attr_count as int)
            && !SseAttrValueSatisfiesStateRules(old_s, event_id, (base_attr_id as int) + i, SbiSharedMemReadXlen(old_s, input_phys_lo, input_phys_hi, xb * ((base_attr_id as int) + i)))
            && (forall|j: int| 0 <= j < i ==> (
                SseAttrValueIsLegal(old_s, event_id, (base_attr_id as int) + j, SbiSharedMemReadXlen(old_s, input_phys_lo, input_phys_hi, xb * ((base_attr_id as int) + j)))
                && !SseAttrIsReadOnly(old_s, event_id, (base_attr_id as int) + j)
                && SseAttrValueSatisfiesStateRules(old_s, event_id, (base_attr_id as int) + j, SbiSharedMemReadXlen(old_s, input_phys_lo, input_phys_hi, xb * ((base_attr_id as int) + j))))))))
    && (result.error == SBI_ERR_BAD_RANGE ==> (exists|i: int| 0 <= i < (attr_count as int) && SseAttrIdIsReserved((base_attr_id as int) + i)))
    && (result.error == SBI_ERR_INVALID_ADDRESS ==> ((input_phys_lo as int) % xb != 0 || !SbiSharedMemIsValid(old_s, input_phys_lo, input_phys_hi, xb * (attr_count as int))))
    && (result.error == SBI_SUCCESS ==> (
        (forall|i: int| 0 <= i < (attr_count as int) ==> (
            (SseEventIsGlobal(old_s, event_id) ==> (forall|h: int| SbiHartIsValid(old_s, h) ==>
                SseEventAttr(new_s, h, event_id, (base_attr_id as int) + i) == SbiSharedMemReadXlen(old_s, input_phys_lo, input_phys_hi, xb * ((base_attr_id as int) + i))))
            && (!SseEventIsGlobal(old_s, event_id) ==>
                SseEventAttr(new_s, SbiCurrentHart(old_s), event_id, (base_attr_id as int) + i) == SbiSharedMemReadXlen(old_s, input_phys_lo, input_phys_hi, xb * ((base_attr_id as int) + i)))))
        && (forall|h: int, e: UInt32, a: int|
            !(e == event_id && (base_attr_id as int) <= a < (base_attr_id as int) + (attr_count as int)
              && (SseEventIsGlobal(old_s, event_id) || h == SbiCurrentHart(old_s)))
            ==> SseEventAttr(new_s, h, e, a) == SseEventAttr(old_s, h, e, a))))
}
