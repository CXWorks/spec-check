pub open spec fn ffa_success_spec(target_info: UInt32, target_id: UInt16, target_vcpu: UInt16, results: [UInt32; 6], old_s: S, new_s: S) -> bool {
  ResultsDeliveredTo(new_s, PreviousInvoker(), results)
}