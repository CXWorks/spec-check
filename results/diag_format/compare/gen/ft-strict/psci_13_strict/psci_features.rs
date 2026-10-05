pub open spec fn psci_features_spec(function_id: UInt32, psci_func_id: UInt32, result: Int32, old_s: S, new_s: S) -> bool {
  (!IsImplementedFunction(old_s, psci_func_id) ==> ResultEqual(result, NOT_SUPPORTED))
  && (CallerIsAArch32(old_s) && IsSmc64FunctionId(old_s, psci_func_id) && IsImplementedFunction(old_s, psci_func_id) ==> ResultEqual(result, NOT_SUPPORTED) || IsValidFeatureFlags(old_s, psci_func_id, result))
  && (Bits(result, 31, 31) == 0)
  && ((psci_func_id == 0x84000001 || psci_func_id == 0xC4000001) ==> Bits(result, 31, 2) == 0)
  && ((psci_func_id == 0x84000001 || psci_func_id == 0xC4000001) ==> ((Bits(result, 1, 1) == 1) == UsesExtendedStateIdFormat()))
  && ((psci_func_id == 0x84000001 || psci_func_id == 0xC4000001) ==> ((Bits(result, 0, 0) == 1) == SupportsOsInitiatedMode()))
  && ((psci_func_id == 0x84000015 || psci_func_id == 0xC4000015) ==> (Bits(result, 31, 31) == 0 && Bits(result, 30, 1) == 0))
  && ((psci_func_id == 0x84000015 || psci_func_id == 0xC4000015) ==> Bits(result, 0, 0) == 1)
  && ((psci_func_id != 0x84000001 && psci_func_id != 0xC4000001 && psci_func_id != 0x84000015 && psci_func_id != 0xC4000015) ==> Bits(result, 31, 0) == 0)
  && ((!(IsImplementedFunction(old_s, psci_func_id)) &&
       !(CallerIsAArch32(old_s) && IsSmc64FunctionId(old_s, psci_func_id) && IsImplementedFunction(old_s, psci_func_id)))
    ==> result == NOT_SUPPORTED)
}