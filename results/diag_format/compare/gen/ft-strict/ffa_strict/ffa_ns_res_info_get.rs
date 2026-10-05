pub open spec fn ffa_ns_res_info_get_spec(target_id: UInt64, flags: UInt64, result: Result<(), FfaStatusCode>, res_info_len: UInt64, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_NS_RES_INFO_GET) ==> ResultEqual(result, NOT_SUPPORTED))
  && (Bits(target_id, 63, 16) != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(flags, 0, 0) == 0 && Bits(target_id, 15, 0) != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(flags, 63, 5) != 0 || Bits(flags, 1, 1) != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(flags, 3, 2) != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!ReservedRegistersAreZero(old_s, 3, 17) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(flags, 0, 0) == 1 && !IsValidSEndpointId(old_s, Bits(target_id, 15, 0)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsRxBufferMappedInCallee(old_s, caller) ==> ResultEqual(result, RETRY))
  && (!IsRxBufferOwnedByCallee(old_s, caller) ==> ResultEqual(result, RETRY))
  && (IsCalleeBusy(old_s) ==> ResultEqual(result, RETRY))
  && (Bits(flags, 4, 4) == 1 && RetrievalAbortedImpDef(old_s, caller) ==> ResultEqual(result, ABORTED))
  && (result == FFA_SUCCESS ==> Bits(res_info_len, 63, 32) == ResInfoDescBytesWrittenToRx(new_s, caller))
  && (result == FFA_SUCCESS ==> Bits(res_info_len, 31, 0) == ResInfoDescBytesRemaining(new_s, caller))
  && (result == FFA_SUCCESS ==> ResInfoDesc(new_s, caller).amd_array_offset % 16 == 0)
  && (result == FFA_SUCCESS ==> Bits(flags, 0, 0) == 0 ==> (forall e: SEndpoint; forall r: NsRegion; IsAccessible(e, r) ==> RegionDescribedByAmd(ResInfoDesc(new_s, caller), e, r)))
  && (result == FFA_SUCCESS ==> Bits(flags, 0, 0) == 1 ==> (forall r: NsRegion; IsAccessible(Bits(target_id, 15, 0), r) ==> RegionDescribedByAmd(ResInfoDesc(new_s, caller), Bits(target_id, 15, 0), r)))
  && (result == FFA_SUCCESS ==> forall a: Amd; (AmdInDesc(ResInfoDesc(new_s, caller), a) && IsIndirectlyAccessibleRegion(a)) ==> (Bits(a.flags, 0, 0) == 1 && a.component_id == SPMC_ID && a.rap == MostPermissiveSpPermission(a)))
  && (result == FFA_SUCCESS ==> forall e: SEndpoint; EntireNsPasAccessibleRwx(e) ==> (exists a: Amd; AmdInDesc(ResInfoDesc(new_s, caller), a) && a.component_id == e && a.address == 0xFFFFFFFFFFFFFFFF && a.page_count == 0 && a.rap == 0x77))
  && (result == FFA_SUCCESS ==> forall e: SEndpoint; forall r: NsRegion; !IsAccessible(e, r) ==> (!RegionDescribedByAmd(ResInfoDesc(new_s, caller), e, r) || AmdRapForRegion(ResInfoDesc(new_s, caller), e, r) == 0))
  && (result == FFA_SUCCESS ==> forall a: Amd; (AmdInDesc(ResInfoDesc(new_s, caller), a) && IsPhysicalSEl1SpUnderSEl2Spmc(a.component_id) && IsStaticNsRegion(a)) ==> a.rap == Stage2BasePermToAmdRap(a))
  && (result == FFA_SUCCESS ==> forall a: Amd; (AmdInDesc(ResInfoDesc(new_s, caller), a) && IsPhysicalSEl0Sp(a.component_id) && IsStaticNsRegion(a)) ==> a.rap == UnprivStage1BasePermToAmdRap(a))
  && (result == FFA_SUCCESS ==> (Bits(flags, 3, 2) == 0 && ((Bits(flags, 0, 0) == 1 && EntireNsPasInaccessibleFrom(new_s, Bits(target_id, 15, 0))) || (Bits(flags, 0, 0) == 0 && EntireNsPasInaccessibleFromAll(new_s)))) ==> res_info_len == 0)
  && ((!(IsImplementedAtInstance(old_s, FFA_NS_RES_INFO_GET)) &&
       !(Bits(target_id, 63, 16) != 0) &&
       !(Bits(flags, 0, 0) == 0 && Bits(target_id, 15, 0) != 0) &&
       !(Bits(flags, 63, 5) != 0 || Bits(flags, 1, 1) != 0) &&
       !(Bits(flags, 3, 2) != 0) &&
       ReservedRegistersAreZero(old_s, 3, 17) &&
       !(Bits(flags, 0, 0) == 1 && !IsValidSEndpointId(old_s, Bits(target_id, 15, 0))) &&
       IsRxBufferMappedInCallee(old_s, caller) &&
       IsRxBufferOwnedByCallee(old_s, caller) &&
       !IsCalleeBusy(old_s) &&
       !(Bits(flags, 4, 4) == 1 && RetrievalAbortedImpDef(old_s, caller)))
    ==> ResultEqual(result, FFA_SUCCESS))
  && (result != FFA_SUCCESS
    ==> ResInfoDesc(new_s, caller).amd_array_offset == ResInfoDesc(old_s, caller).amd_array_offset)
  && (result != FFA_SUCCESS
    ==> ResInfoDesc(new_s, caller).amd_array_offset == ResInfoDesc(old_s, caller).amd_array_offset)
  && (result != FFA_SUCCESS
    ==> ResInfoDesc(new_s, caller).amd_array_offset == ResInfoDesc(old_s, caller).amd_array_offset)
  && (result != FFA_SUCCESS
    ==> ResInfoDesc(new_s, caller).amd_array_offset == ResInfoDesc(old_s, caller).amd_array_offset)
  && (result != FFA_SUCCESS
    ==> ResInfoDesc(new_s, caller).amd_array_offset == ResInfoDesc(old_s, caller).amd_array_offset)
  && (result != FFA_SUCCESS
    ==> ResInfoDesc(new_s, caller).amd_array_offset == ResInfoDesc(old_s, caller).amd_array_offset)
  && (result != FFA_SUCCESS
    ==> ResInfoDesc(new_s, caller).amd_array_offset == ResInfoDesc(old_s, caller).amd_array_offset)
  && (result != FFA_SUCCESS
    ==> ResInfoDesc(new_s, caller).amd_array_offset == ResInfoDesc(old_s, caller).amd_array_offset)
  && (result != FFA_SUCCESS
    ==> ResInfoDesc(new_s, caller).amd_array_offset == ResInfoDesc(old_s, caller).amd_array_offset)
  && (result != FFA_SUCCESS
    ==> ResInfoDesc(new_s, caller).amd_array_offset == ResInfoDesc(old_s, caller).amd_array_offset)
}