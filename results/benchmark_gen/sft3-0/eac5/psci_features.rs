pub open spec fn psci_features_spec(psci_func_id: UInt32, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (true ==> result == PSCI_SUCCESS)
  && ((psci_func_id < PSCI_FUNCTION_COUNT) ==> result == PSCI_SUCCESS)
  && (!(psci_func_id < PSCI_FUNCTION_COUNT) ==> result == PSCI_NOT_SUPPORTED)
}