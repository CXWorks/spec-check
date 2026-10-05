pub open spec fn ffa_abort_spec(result: (), old_s: S, new_s: S) -> bool {
    (ExecutionContextState(new_s, caller) == ABORTED)
    && (InvocationDoesNotComplete())
}