pub open spec fn rmi_vdev_validate_mapping_spec(rd: Rd, rec_ptr: PhysAddr, pdev_ptr: PhysAddr, vdev_ptr: PhysAddr, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  ((!(is_delegable_physical_address(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (!(GranuleAt(old_s, rd).state == RD_STATE) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (!(is_delegable_physical_address(old_s, rec_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (!(GranuleAt(old_s, rec_ptr).state == REC_STATE) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (RealmAt(old_s, rd).rec_state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
   && (!is_rec_owned_by_realm(old_s, rd, rec_ptr) ==> ResultEqual(result, RMI_ERROR_REC))
   && (!(is_delegable_physical_address(old_s, pdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (!(GranuleAt(old_s, pdev_ptr).state == PDEV_STATE) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (!(is_delegable_physical_address(old_s, vdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (!(GranuleAt(old_s, vdev_ptr).state == VDEV_STATE) ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (!is_vdev_associated_with_pdev(old_s, vdev_ptr, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_DEVICE))
   && (top <= base ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (RealmAt(old_s, rd).dev_mem_addr != base ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (top > RealmAt(old_s, rd).dev_mem_top ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (!(base % RTT_ENTRY_SIZE(RealmAt(old_s, rd).rtt_level_reached as int)) ==> ResultEqual(result, RMI_ERROR_RTT))
   && (top % GRANULE_SIZE != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
   && (base == top_of_walked_range(old_s, rd) ==> ResultEqual(result, RMI_ERROR_RTT))
   && (RealmAt(old_s, rd).dev_mem_flags == DEV_MEM_NON_COHERENT && exists_entry_in_walked_range(old_s, rd) ==> ResultEqual(result, RMI_ERROR_RTT))
   && (RealmAt(old_s, rd).dev_mem_flags == DEV_MEM_NON_COHERENT && exists_entry_in_walked_range(old_s, rd) ==> ResultEqual(result, RMI_ERROR_RTT))
   && (RealmAt(old_s, rd).dev_mem_flags == DEV_MEM_COHERENT && exists_entry_in_walked_range(old_s, rd) ==> ResultEqual(result, RMI_ERROR_RTT))
   && (RealmAt(old_s, rd).dev_mem_flags == DEV_MEM_COHERENT && exists_entry_in_walked_range(old_s, rd) ==> ResultEqual(result, RMI_ERROR_RTT))
   && (exists_non_contiguous_output_addresses(old_s, rd) ==> ResultEqual(result, RMI_ERROR_RTT))
   && (exists_address_live_in_aux_rtt(old_s, rd, base, top) ==> ResultEqual(result, RMI_ERROR_RTT))
   && (result.is_Ok() ==> (RealmAt(new_s, rd).rtt_entries[RealmAt(new_s, rd).rtt_level_reached as int].ripas == DEV))
   && (result.is_Ok() ==> (RealmAt(new_s, rd).dev_mem_addr == min(top, top_of_walked_range(new_s, rd))))
   && (result.is_Ok() ==> (RealmAt(new_s, rd).dev_mem_pa == RealmAt(old_s, rd).dev_mem_pa + (top - base)))
   && (result.is_Ok() ==> (out_top == min(top, top_of_walked_range(new_s, rd))))
   && ((is_delegable_physical_address(old_s, rd) &&
        GranuleAt(old_s, rd).state == RD_STATE &&
        is_delegable_physical_address(old_s, rec_ptr) &&
        GranuleAt(old_s, rec_ptr).state == REC_STATE &&
        !(RealmAt(old_s, rd).rec_state == REC_RUNNING) &&
        is_rec_owned_by_realm(old_s, rd, rec_ptr) &&
        is_delegable_physical_address(old_s, pdev_ptr) &&
        GranuleAt(old_s, pdev_ptr).state == PDEV_STATE &&
        is_delegable_physical_address(old_s, vdev_ptr) &&
        GranuleAt(old_s, vdev_ptr).state == VDEV_STATE &&
        is_vdev_associated_with_pdev(old_s, vdev_ptr, pdev_ptr) &&
        !(top <= base) &&
        RealmAt(old_s, rd).dev_mem_addr == base &&
        !(top > RealmAt(old_s, rd).dev_mem_top) &&
        (base % RTT_ENTRY_SIZE(RealmAt(old_s, rd).rtt_level_reached as int)) &&
        !(top % GRANULE_SIZE != 0) &&
        !(base == top_of_walked_range(old_s, rd)) &&
        !(RealmAt(old_s, rd).dev_mem_flags == DEV_MEM_NON_COHERENT && exists_entry_in_walked_range(old_s, rd)) &&
        !(RealmAt(old_s, rd).dev_mem_flags == DEV_MEM_NON_COHERENT && exists_entry_in_walked_range(old_s, rd)) &&
        !(RealmAt(old_s, rd).dev_mem_flags == DEV_MEM_COHERENT && exists_entry_in_walked_range(old_s, rd)) &&
        !(RealmAt(old_s, rd).dev_mem_flags == DEV_MEM_COHERENT && exists_entry_in_walked_range(old_s, rd)) &&
        !(exists_non_contiguous_output_addresses(old_s, rd)) &&
        !(exists_address_live_in_aux_rtt(old_s, rd, base, top)))
     ==> result.is_Ok())
   && (result.is_Err()
     ==> RealmAt(new_s, rd).rtt_entries[RealmAt(new_s, rd).rtt_level_reached as int].ripas == RealmAt(old_s, rd).rtt_entries[RealmAt(old_s, rd).rtt_level_reached as int].ripas)
   && (result.is_Err()
     ==> RealmAt(new_s, rd).dev_mem_addr == RealmAt(old_s, rd).dev_mem_addr)
   && (result.is_Err()
     ==> RealmAt(new_s, rd).dev_mem_pa == RealmAt(old_s, rd).dev_mem_pa)
   && (result.is_Err()
     ==> RealmAt(new_s, rd).dev_mem_addr == RealmAt(old_s, rd).dev_mem_addr)
}