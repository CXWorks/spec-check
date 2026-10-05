pub open spec fn sbi_probe_extension_spec(extension_id: Int64, value: Int64, old_s: S, new_s: S) -> bool {
  (!IsExtensionAvailable(old_s, extension_id) ==> value == 0)
  && (IsExtensionAvailable(old_s, extension_id) ==> value != 0)
  && (IsExtensionAvailable(old_s, extension_id) && !ImplDefinesProbeValue(old_s, extension_id) ==> value == 1)
  && (IsExtensionAvailable(old_s, extension_id) && ImplDefinesProbeValue(old_s, extension_id) ==> value == ImplProbeValue(old_s, extension_id))
  && ((!(IsExtensionAvailable(old_s, extension_id)))
    ==> value == 0)
}