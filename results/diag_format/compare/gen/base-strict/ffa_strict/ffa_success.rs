pub open spec fn ffa_success_spec(fid: UInt32, target_info: UInt32, results: [UInt32; 8], old_s: S, new_s: S) -> bool {
    (ResultsReturnedToInvoker(PreviousInvocation(), results) &&
     (ConduitIsSmc(old_s) && InstanceIsNonSecureVirtual(old_s) ==> ResultsDeliveredToVcpu(Bits(target_info, 31, 16), Bits(target_info, 15, 0), results)) &&
     (!(ConduitIsSmc(old_s) && InstanceIsNonSecureVirtual(old_s)) ==> target_info == 0) &&
     (fid == 0xC4000061 ==> (AnyResultIs64Bit(results) || CallerImplementsOnlySmc64Fids())))
}