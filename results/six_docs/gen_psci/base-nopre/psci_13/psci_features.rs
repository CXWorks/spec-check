pub open spec fn psci_features_spec(result: u64, old_s: S, new_s: S) -> bool {
    let psci_func_id = old_s.psci_func_id;
    let psci_version = old_s.psci_version;
    let is_smcc_version = psci_version >= 1;
    let is_smc64_function = psci_func_id >= 0xC400_0000;
    let is_smc32_function = psci_func_id >= 0x8400_0000 && psci_func_id < 0xC400_0000;
    let is_cpu_suspend = psci_func_id == 0x8400_0001 || psci_func_id == 0xC400_0001;
    let is_system_off2 = psci_func_id == 0x8400_0015 || psci_func_id == 0xC400_0015;
    let is_other_function = !is_cpu_suspend && !is_system_off2;
    let is_not_supported = result == 0;
    let is_feature_flags = result > 0;
    let bit_31 = (result >> 31) as u64;
    let bit_1 = (result >> 1) as u64;
    let bit_0 = (result >> 0) as u64;
    let bits_30_0 = result & 0x7FFFFFFF;
    let bits_30_1 = (bits_30_0 >> 1) & 0x3FFFFFFF;
    let bit_0_system_off2 = bits_30_0 & 0x1;
    (!is_smcc_version ==> is_not_supported)
    && (is_smcc_version ==> (is_not_supported || is_feature_flags))
    && (is_smc64_function ==> is_not_supported)
    && (is_smc32_function ==> (is_not_supported || is_feature_flags))
    && (is_feature_flags ==> (bit_31 == 0))
    && (is_cpu_suspend ==> (bit_31 == 0 && bit_1 == 0 || bit_1 == 1 && bit_0 == 0 || bit_1 == 1 && bit_0 == 1))
    && (is_system_off2 ==> (bit_31 == 0 && bit_0_system_off2 == 1))
    && (is_other_function ==> (is_not_supported || (is_feature_flags && bit_31 == 0)))
}