pub open spec fn ffa_abort_spec(result: (), old_s: S, new_s: S) -> bool {
    (ExecutionContextIsAborted(new_s, CallerExecutionContext(old_s)))
    && (!InvocationCompletes(old_s.fid))
}