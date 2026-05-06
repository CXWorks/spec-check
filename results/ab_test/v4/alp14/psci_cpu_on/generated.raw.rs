pub open spec fn PSCI_CPU_ON_spec(
    old_s: S,
    new_s: S,
    target_cpu: Bits64,
    entry_point_address: Address,
    context_id: UInt32,
    result: PsciReturnCode
) -> bool {
    let realm = CurrentRealm(old_s);
    let target_rec = RecFromMpidr(old_s, target_cpu);
    
    ((!AddrIsProtected(old_s, entry_point_address, realm) ==> result == PSCI_INVALID_ADDRESS) &&
     (!MpidrIsUsed(old_s, target_cpu) ==> result == PSCI_INVALID_PARAMETERS) &&
     (target_rec.flags.runnable == RUNNABLE ==> result == PSCI_ALREADY_ON)) &&
    
    ((AddrIsProtected(old_s, entry_point_address, realm) && 
      MpidrIsUsed(old_s, target_cpu) && 
      target_rec.flags.runnable != RUNNABLE &&
      result == PSCI_SUCCESS) ==>
     (RecFromMpidr(new_s, target_cpu).pc == ToBits64(UInt(entry_point_address) as int) &&
      RecFromMpidr(new_s, target_cpu).flags.runnable == RUNNABLE))
}