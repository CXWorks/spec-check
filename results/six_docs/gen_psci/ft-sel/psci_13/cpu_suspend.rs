pub open spec fn cpu_suspend_spec(power_state: u64, entry_point_address: Address, context_id: ContextId, result: Result<int, PsciStatusCode>, old_s: S, new_s: S) -> bool {
  true
}