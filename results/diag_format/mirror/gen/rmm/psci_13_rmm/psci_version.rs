pub open spec fn psci_version_spec(result: Result<(), NotSupported>, old_s: S, new_s: S) -> bool {
  (!IsFunctionImplemented(old_s, PSCI_VERSION) ==> result)
  && (result == NOT_SUPPORTED ==> result)
  && (result == NOT_SUPPORTED ==> 0)
  && (result == NOT_SUPPORTED ==> 0)
  && (result == NOT_SUPPORTED ==> 0)
  && ((result == NOT_SUPPORTED) ==> (result[30:16] == ImplementedPsciVersion().major))
  && ((result == NOT_SUPPORTED) ==> (result[15:0] == ImplementedPsciVersion().minor))
  && ((result == NOT_SUPPORTED) ==> (result[31] == 0))
  && (result == NOT_SUPPORTED ==> true)
}