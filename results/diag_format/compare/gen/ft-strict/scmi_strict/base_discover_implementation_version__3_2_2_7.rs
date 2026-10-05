pub open spec fn base_discover_implementation_version__3_2_2_7_spec(implementation_version: uint32, old_s: S, new_s: S) -> bool {
  (implementation_version == VendorImplementationVersion())
  && (forall (v: UInt32), IsEarlierImplementationVersion(v) ==> v < implementation_version)
  && (true)
}