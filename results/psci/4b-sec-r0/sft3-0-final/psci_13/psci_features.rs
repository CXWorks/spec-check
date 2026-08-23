pub open spec fn psci_features_spec(func_id: UInt32, result: int, old_s: S, new_s: S) -> bool {
  (result == PSCI_NOT_SUPPORTED ==> (result == PSCI_NOT_SUPPORTED))
  && ((!(result == PSCI_NOT_SUPPORTED)) ==> (result == PSCI_SUCCESS))
}