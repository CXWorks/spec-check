pub open spec fn PSCI_FEATURES_spec(s: S, psci_func_id: u32, result: PsciReturnCode) -> bool {
    if is_supported_psci_function(s, psci_func_id) {
        result == PsciReturnCode::PSCI_SUCCESS
    } else {
        result == PsciReturnCode::PSCI_NOT_SUPPORTED
    }
}