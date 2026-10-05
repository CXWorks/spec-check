pub open spec fn rmi_rtt_set_ripas_spec(rd: Rd, rec_ptr: PhysAddr, base: UInt64, top: UInt64, result: Result<(), RmiStatusCode>, out_top: UInt64, old_s: S, new_s: S) -> bool {
  (rd as int) % GRANULE_SIZE == 0 ==> ResultEqual(result, RMI_ERROR_INPUT)
  && (is_delegable_physical_address(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleState(old_s, rd) != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (rec_ptr % GRANULE_SIZE == 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (is_delegable_physical_address(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleState(old_s, rec_ptr) != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result == RMI_SUCCESS && RealmAt(old_s, rd).rec_state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (result == RMI_SUCCESS && !REC_owns_Realm(old_s, rd, rec_ptr) ==> ResultEqual(result, RMI_ERROR_REC))
  && (top <= base ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result == RMI_SUCCESS && RipasAddr(RealmAt(old_s, rd).recs[rec_ptr as int]) != base ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result == RMI_SUCCESS && top > RipasTop(RealmAt(old_s, rd).recs[rec_ptr as int]) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result == RMI_SUCCESS && !AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, rd, 0 as int).level as int) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, rd, 0 as int).level as int)))
  && (result == RMI_SUCCESS && RipasAt(old_s, base) != RipasValue(RealmAt(old_s, rd).recs[rec_ptr as int]) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, rd, 0 as int).level as int)))
  && (result == RMI_SUCCESS && top % GRANULE_SIZE != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result == RMI_SUCCESS && base == RipasTop(RealmAt(old_s, rd).recs[rec_ptr as int]) && RipasAt(old_s, base) != RipasValue(RealmAt(old_s, rd).recs[rec_ptr as int]) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, rd, 0 as int).level as int)))
  && (result == RMI_SUCCESS && IsAuxRtt(old_s, base, top, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, rd, 0 as int).level as int)))
  && (result.is_Ok() ==> RipasAddr(new_s, RealmAt(new_s, rd).recs[rec_ptr as int]) == min(top, RttWalk(new_s, rd, 0 as int).top))
  && (result.is_Ok() ==> out_top == min(top, RttWalk(new_s, rd, 0 as int).top))
  && ((!( (rd as int) % GRANULE_SIZE == 0) &&
       is_delegable_physical_address(old_s, rd) &&
       !(GranuleState(old_s, rd) != RD) &&
       !((rec_ptr % GRANULE_SIZE == 0) && is_delegable_physical_address(old_s, rec_ptr)) &&
       !(GranuleState(old_s, rec_ptr) != REC))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RipasAddr(new_s, RealmAt(new_s, rd).recs[rec_ptr as int]) == RipasAddr(old_s, RealmAt(old_s, rd).recs[rec_ptr as int]))
  && (result.is_Err()
    ==> RipasTop(new_s, RealmAt(new_s, rd).recs[rec_ptr as int]) == RipasTop(old_s, RealmAt(old_s, rd).recs[rec_ptr as int]))
}