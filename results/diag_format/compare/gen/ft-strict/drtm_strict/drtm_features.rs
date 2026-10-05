pub open spec fn drtm_features_spec(id: UInt64, result: Int64, features: Bits64, old_s: S, new_s: S) -> bool {
  ((!(Bits(id, 63, 63) == 0 && !IsImplementedDrtmFunction(Bits(id, 31, 0)))) ==> result != NOT_SUPPORTED)
  && ((!(Bits(id, 63, 63) == 1 && !IsSupportedDrtmFeature(Bits(id, 7, 0)))) ==> result != NOT_SUPPORTED)
  && (result >= 0)
  && ((Bits(id, 63, 63) == 1 && Bits(id, 7, 0) == 1 && result > 0) ==> (Bits(features, 63, 37) == 0 && Bits(features, 36, 33) == SupportedPcrUsageSchemas() && Bits(features, 32, 32) == TpmBasedHashSupported() && Bits(features, 15, 0) == FirmwareHashAlgorithmId()))
  && ((Bits(id, 63, 63) == 1 && Bits(id, 7, 0) == 2 && result > 0) ==> (Bits(features, 63, 32) == NormalWorldDceMinPages() && Bits(features, 31, 0) == DlmeDataMinPages()))
  && ((Bits(id, 63, 63) == 1 && Bits(id, 7, 0) == 2 && result > 0 && !UsesNormalWorldDce()) ==> Bits(features, 63, 32) == 0)
  && ((Bits(id, 63, 63) == 1 && Bits(id, 7, 0) == 3 && result > 0) ==> (Bits(features, 63, 24) == 0 && Bits(features, 7, 0) == DmaProtectionSupportBitmap()))
  && ((Bits(id, 63, 63) == 1 && Bits(id, 7, 0) == 3 && result > 0 && RegionBasedDmaProtectionSupported()) ==> (Bits(features, 23, 8) == MaxDmaProtectedRegions() && Bits(features, 23, 8) != 0))
  && ((Bits(id, 63, 63) == 1 && Bits(id, 7, 0) == 4 && result > 0) ==> features == BootPeTargetCpuEncoding())
  && ((Bits(id, 63, 63) == 1 && Bits(id, 7, 0) == 5 && result > 0) ==> (Bits(features, 63, 8) == 0 && Bits(features, 7, 0) == MaxTcbHashes()))
  && ((Bits(id, 63, 63) == 1 && Bits(id, 7, 0) == 6 && result > 0) ==> (Bits(features, 63, 1) == 0 && Bits(features, 0, 0) == DlmeImageAuthSupported()))
  && ((!( (Bits(id, 63, 63) == 0 && IsImplementedDrtmFunction(Bits(id, 31, 0))) ||
         (Bits(id, 63, 63) == 1 && IsSupportedDrtmFeature(Bits(id, 7, 0))))
    ==> result == 0)
}