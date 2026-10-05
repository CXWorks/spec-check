pub open spec fn sbi_probe_extension_spec(extension_id: long, ret: struct sbiret, old_s: S, new_s: S) -> bool {
  (!IsExtensionAvailable(old_s, extension_id) ==> ret == 0)
  && (IsExtensionAvailable(old_s, extension_id) ==> (ret == 1) || (ret != 0 && IsImplementationDefinedProbeValue(ret)))
  && ((!(IsExtensionAvailable(old_s, extension_id)))
    ==> ret == 0)
}