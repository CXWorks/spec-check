pub open spec fn sbi_sse_hart_unmask_spec(result: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (SseHartUnmasked(old_s, CurrentHartId(old_s)) ==> (result == SBI_ERR_ALREADY_STARTED && SseHartUnmasked(new_s, CurrentHartId(old_s))))
    && (!SseHartUnmasked(old_s, CurrentHartId(old_s)) ==> ((result == SBI_SUCCESS && SseHartUnmasked(new_s, CurrentHartId(old_s))) || (result == SBI_ERR_FAILED && !SseHartUnmasked(new_s, CurrentHartId(old_s)))))
    && (result == SBI_SUCCESS ==> (!SseHartUnmasked(old_s, CurrentHartId(old_s)) && SseHartUnmasked(new_s, CurrentHartId(old_s))))
    && (result != SBI_SUCCESS ==> SseHartUnmasked(new_s, CurrentHartId(old_s)) == SseHartUnmasked(old_s, CurrentHartId(old_s)))
}
