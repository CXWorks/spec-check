pub open spec fn ffa_success_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS ==> ResultsDeliveredTo(PreviousInvoker(), results))
}