pub open spec fn psci_features_spec(result: Int32, psci_func_id: UInt32, old_s: S, new_s: S) -> bool {
    (!IsImplementedFunction(psci_func_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (CallerIsAArch32() && IsSmc64FunctionId(psci_func_id) && IsImplementedFunction(psci_func_id) ==> (ResultEqual(result, NOT_SUPPORTED) || IsValidFeatureFlags(psci_func_id, result)))
    && (Bits(result, 31, 31) == 0)
    && ((psci_func_id == 0x84000001 || psci_func_id == 0xC4000001) ==> (Bits(result, 31, 2) == 0))
    && ((psci_func_id == 0x84000001 || psci_func_id == 0xC4000001) ==> ((Bits(result, 1, 1) == 1) == UsesExtendedStateIdFormat()))
    && ((psci_func_id == 0x84000001 || psci_func_id == 0xC4000001) ==> ((Bits(result, 0, 0) == 1) == SupportsOsInitiatedMode()))
    && ((psci_func_id == 0x84000015 || psci_func_id == 0xC4000015) ==> (Bits(result, 31, 31) == 0 && Bits(result, 30, 1) == 0))
    && ((psci_func_id == 0x84000015 || psci_func_id == 0xC4000015) ==> Bits(result, 0, 0) == 1)
    && ((psci_func_id != 0x84000001 && psci_func_id != 0xC4000001 && psci_func_id != 0x84000015 && psci_func_id != 0xC4000015) ==> Bits(result, 31, 0) == 0)
    && (old_s == new_s)
}