pub open spec fn ffa_success_spec(target_info: UInt32, target_id: UInt16, target_vcpu_id: UInt16, results: [UInt32; 6], old_s: S, new_s: S) -> bool {
  ResultsReturnedToInvoker(old_s, PreviousInvocation(old_s), results)
  && ((ConduitIsSmc(old_s) && InstanceIsNonSecureVirtual(old_s)) ==> ResultsDeliveredToVcpu(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0), results))
  && (!(ConduitIsSmc(old_s) && InstanceIsNonSecureVirtual(old_s)) ==> target_info == 0)
  && (fid == 0xC4000061 ==> (AnyResultIs64Bit(results) || CallerImplementsOnlySmc64Fids()))
  && ((!(ConduitIsSmc(old_s) && InstanceIsNonSecureVirtual(old_s)))
    ==> target_info == 0)
}