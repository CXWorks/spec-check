pub open spec fn rmi_vdev_validate_mapping_spec(rd: Address, rec_ptr: Address, pdev_ptr: Address, vdev_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (is_aligned_to(GranuleSize, rd) ==> result.is_Ok())
  && (is_delegable_physical_address(rd) ==> result.is_Ok())
  && (GranuleAt(old_s, rd).state == RD ==> result.is_Ok())
  && (is_aligned_to(GranuleSize, rec_ptr) ==> result.is_Ok())
  && (is_delegable_physical_address(rec_ptr) ==> result.is_Ok())
  && (GranuleAt(old_s, rec_ptr).state == REC ==> result.is_Ok())
  && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (RecAt(old_s, rec_ptr).owner != RealmAt(old_s, rd) ==> ResultEqual(result, RMI_ERROR_REC))
  && (is_aligned_to(GranuleSize, pdev_ptr) ==> result.is_Ok())
  && (is_delegable_physical_address(pdev_ptr) ==> result.is_Ok())
  && (GranuleAt(old_s, pdev_ptr).state == PDEV ==> result.is_Ok())
  && (is_aligned_to(GranuleSize, vdev_ptr) ==> result.is_Ok())
  && (is_delegable_physical_address(vdev_ptr) ==> result.is_Ok())
  && (GranuleAt(old_s, vdev_ptr).state == VDEV ==> result.is_Ok())
  && (VdevAt(old_s, vdev_ptr).pdev != pdev_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (top <= base ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, rec_ptr).dev_mem_addr != base ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (top > RecAt(old_s, rec_ptr).dev_mem_top ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!is_aligned_to(RttEntrySize(RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).level as int)), base) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int))
  && (!is_aligned_to(GranuleSize, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (base == RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).rtt_addr,RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,false).0 ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int))
  && (RecAt(old_s, rec_ptr).dev_mem_flags == DEV_MEM_NON_COHERENT && !every_rtt_entry_in_walked_range_has(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int), MEMATTR_NON_CACHEABLE) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int))
  && (RecAt(old_s, rec_ptr).dev_mem_flags == DEV_MEM_NON_COHERENT && !every_rtt_entry_in_walked_range_maps_non_coherent_device_memory(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int))
  && (RecAt(old_s, rec_ptr).dev_mem_flags == DEV_MEM_COHERENT && !every_rtt_entry_in_walked_range_has(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int), MEMATTR_PASSTHROUGH) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int))
  && (RecAt(old_s, rec_ptr).dev_mem_flags == DEV_MEM_COHERENT && !every_rtt_entry_in_walked_range_maps_coherent_device_memory(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int))
  && (!contiguous_output_addresses_of_rtt_entries_in_walked_range(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int), RecAt(new_s, rec_ptr).dev_mem_pa) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int))
  && (any_address_in_range_from_base_to_top_is_live_in_auxiliary_rtt(RealmAt(new_s, rd), base, top) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int))
  && (result.is_Ok() ==> (RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).ripas == DEV))
  && (result.is_Ok() ==> (RecAt(new_s, rec_ptr).dev_mem_addr == min(top, RttSkipEntriesWithRipas(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).rtt_addr,RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,false).0)))
  && (result.is_Ok() ==> (RecAt(new_s, rec_ptr).dev_mem_pa == RecAt(old_s, rec_ptr).dev_mem_pa + (RttSkipEntriesWithRipas(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).rtt_addr,RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,false).1 - base)))
  && (result.is_Ok() ==> (out_top == min(top, RttSkipEntriesWithRipas(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).rtt_addr,RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,false).0)))
  && ((!(is_aligned_to(GranuleSize, rd)) ||
       !(is_delegable_physical_address(rd)) ||
       !(GranuleAt(old_s, rd).state == RD) ||
       !(is_aligned_to(GranuleSize, rec_ptr)) ||
       !(is_delegable_physical_address(rec_ptr)) ||
       !(GranuleAt(old_s, rec_ptr).state == REC) ||
       !(RecAt(old_s, rec_ptr).state == REC_RUNNING) ||
       !(RecAt(old_s, rec_ptr).owner != RealmAt(old_s, rd)) ||
       !(is_aligned_to(GranuleSize, pdev_ptr)) ||
       !(is_delegable_physical_address(pdev_ptr)) ||
       !(GranuleAt(old_s, pdev_ptr).state == PDEV) ||
       !(is_aligned_to(GranuleSize, vdev_ptr)) ||
       !(is_delegable_physical_address(vdev_ptr)) ||
       !(GranuleAt(old_s, vdev_ptr).state == VDEV) ||
       !(VdevAt(old_s, vdev_ptr).pdev != pdev_ptr) ||
       !(top <= base) ||
       !(RecAt(old_s, rec_ptr).dev_mem_addr != base) ||
       !(top > RecAt(old_s, rec_ptr).dev_mem_top) ||
       is_aligned_to(RttEntrySize(RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).level as int)), base) ||
       is_aligned_to(GranuleSize, top) ||
       !(base == RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).rtt_addr,RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,false).0) ||
       !(RecAt(old_s, rec_ptr).dev_mem_flags == DEV_MEM_NON_COHERENT && every_rtt_entry_in_walked_range_has(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int), MEMATTR_NON_CACHEABLE)) ||
       !(RecAt(old_s, rec_ptr).dev_mem_flags == DEV_MEM_NON_COHERENT && every_rtt_entry_in_walked_range_maps_non_coherent_device_memory(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int))) ||
       !(RecAt(old_s, rec_ptr).dev_mem_flags == DEV_MEM_COHERENT && every_rtt_entry_in_walked_range_has(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int), MEMATTR_PASSTHROUGH)) ||
       !(RecAt(old_s, rec_ptr).dev_mem_flags == DEV_MEM_COHERENT && every_rtt_entry_in_walked_range_maps_coherent_device_memory(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int))) ||
       contiguous_output_addresses_of_rtt_entries_in_walked_range(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int), RecAt(new_s, rec_ptr).dev_mem_pa) ||
       !(any_address_in_range_from_base_to_top_is_live_in_auxiliary_rtt(RealmAt(new_s, rd), base, top)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).dev_mem_addr == RecAt(old_s, rec_ptr).dev_mem_addr)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).dev_mem_pa == RecAt(old_s, rec_ptr).dev_mem_pa)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).dev_mem_top == RecAt(old_s, rec_ptr).dev_mem_top)
}