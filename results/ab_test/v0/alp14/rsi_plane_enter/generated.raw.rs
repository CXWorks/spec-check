```verus
pub open spec fn RSI_PLANE_ENTER_spec(s: S, plane_idx: u64, run_ptr: Address, realm: RmmRealm) -> bool {
  let run = RsiPlaneRunAt(s, realm, run_ptr);
  let walk = RttWalk(s, realm, run_ptr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
  
  (plane_idx == 0 || plane_idx > realm.num_aux_planes) ==> false &&
  !AddrIsGranuleAligned(run_ptr) ==> false &&
  !AddrIsProtected(run_ptr, realm) ==> false &&
  walk.rtte.ripas == EMPTY ==> false &&
  run.enter.spsr_el2[3] == 1 ==> false &&
  (plane_idx > 0 && plane_idx <= realm.num_aux_planes &&
   AddrIsGranuleAligned(run_ptr) &&
   AddrIsProtected(run_ptr, realm) &&
   walk.rtte.ripas != EMPTY &&
   run.enter.spsr_el2[3] == 0) ==> true
}
```