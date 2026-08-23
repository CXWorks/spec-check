pub open spec fn cpu_on_spec(entry_point_address: Address, context_identifier: UInt32, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (result == PSCI_INVALID_PARAMETERS ==> true)
  && (result == PSCI_INVALID_ADDRESS ==> true)
  && (result == PSCI_ALREADY_ON ==> true)
  && (result == PSCI_ON_PENDING ==> true)
  && (result == PSCI_INTERNAL_FAILURE ==> true)
  && (result == PSCI_DENIED ==> true)
  && ((!(result == PSCI_INVALID_PARAMETERS) &&
       result == PSCI_SUCCESS)
    ==> true)
}