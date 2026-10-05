pub open spec fn ffa_abort_spec(old_s: S, new_s: S) -> bool {
  (ExecutionContextIsAborted(new_s, CallerExecutionContext(new_s)) &&
   !InvocationCompletes(new_s, 0))
}