pub open spec fn psci_cpu_on_spec(target_cpu: Bits64, entry_point_address: Address, context_id: UInt32, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (!AddrIsProtected(old_s, entry_point_address, CurrentRealm(old_s)) ==> result == PSCI_INVALID_ADDRESS)
  && (!MpidrIsUsed(old_s, target_cpu) ==> result == PSCI_INVALID_PARAMETERS)
  && (RecFromMpidr(old_s, target_cpu).flags.runnable == RUNNABLE ==> result == PSCI_ALREADY_ON)
  && (result == PSCI_SUCCESS
    ==> RecFromMpidr(new_s, target_cpu).pc == ToBits64((entry_point_address) as int))
  && (result == PSCI_SUCCESS
    ==> RecFromMpidr(new_s, target_cpu).flags.runnable == RUNNABLE)
  && ((AddrIsProtected(old_s, entry_point_address, CurrentRealm(old_s)) &&
       MpidrIsUsed(old_s, target_cpu) &&
       !(RecFromMpidr(old_s, target_cpu).flags.runnable == RUNNABLE))
    ==> result == PSCI_SUCCESS)
  && (result != PSCI_SUCCESS
    ==> RecFromMpidr(new_s, target_cpu).pc == RecFromMpidr(old_s, target_cpu).pc)
  && (result != PSCI_SUCCESS
    ==> RecFromMpidr(new_s, target_cpu).flags.runnable == RecFromMpidr(old_s, target_cpu).flags.runnable)
}