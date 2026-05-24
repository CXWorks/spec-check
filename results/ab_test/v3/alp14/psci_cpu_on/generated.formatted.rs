pub open spec fn psci_cpu_on_spec(result: PsciReturnCode, entry_point_address: Address, target_cpu: Bits64, context_id: u32, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    let target_rec_old = RecFromMpidr(old_s, target_cpu);
    let target_rec_new = RecFromMpidr(new_s, target_cpu);
    
    (!AddrIsProtected(old_s, entry_point_address, realm) ==> result == PSCI_INVALID_ADDRESS)
    && (!MpidrIsUsed(old_s, target_cpu) ==> result == PSCI_INVALID_PARAMETERS)
    && (target_rec_old.flags.runnable == RUNNABLE ==> result == PSCI_ALREADY_ON)
    && (AddrIsProtected(old_s, entry_point_address, realm)
        && MpidrIsUsed(old_s, target_cpu)
        && target_rec_old.flags.runnable != RUNNABLE
        ==> (result.is_Ok()
            && target_rec_new.pc == ToBits64(entry_point_address as int)
            && target_rec_new.flags.runnable == RUNNABLE))
}