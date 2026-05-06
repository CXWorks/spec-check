pub open spec fn PSCI_AFFINITY_INFO_spec(s: S, target_affinity: u64, lowest_affinity_level: u32, result: u64) -> bool {
  (lowest_affinity_level != 0 ==> result == PSCI_INVALID_PARAMETERS) &&
  (!MpidrIsUsed(s, target_affinity) ==> result == PSCI_INVALID_PARAMETERS) &&
  (MpidrIsUsed(s, target_affinity) && lowest_affinity_level == 0 ==> 
    (let target_rec = RecFromMpidr(s, target_affinity);
     (target_rec.flags.runnable == RUNNABLE ==> result == PSCI_SUCCESS) &&
     (target_rec.flags.runnable == NOT_RUNNABLE ==> result == PSCI_OFF)))
}