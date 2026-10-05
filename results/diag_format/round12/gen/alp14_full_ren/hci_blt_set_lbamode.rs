pub open spec fn hci_blt_set_lbamode_spec(vd: Vd, worker_ptr: WorkerPtr, base: Address, top: Address, result: Result<(), HciStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  ((vd) % extent_size(old_s) != 0 ==> HCI_ERROR_INPUT(result))
  && (!is_enrollable_physical_address(old_s, vd) ==> HCI_ERROR_INPUT(result))
  && (ExtentAt(old_s, vd).state != VD_STATE ==> HCI_ERROR_INPUT(result))
  && ((worker_ptr) % extent_size(old_s) != 0 ==> HCI_ERROR_INPUT(result))
  && (!is_enrollable_physical_address(old_s, worker_ptr) ==> HCI_ERROR_INPUT(result))
  && (ExtentAt(old_s, worker_ptr).state != WORKER_STATE ==> HCI_ERROR_INPUT(result))
  && (WorkerAt(old_s, worker_ptr).state == WORKER_RUNNING ==> HCI_ERROR_WORKER(result))
  && (!WorkerOwnedByVault(old_s, worker_ptr, vd) ==> HCI_ERROR_WORKER(result))
  && (top <= base ==> HCI_ERROR_INPUT(result))
  && (base != WorkerAt(old_s, worker_ptr).lbamode_addr ==> HCI_ERROR_INPUT(result))
  && (top > WorkerAt(old_s, worker_ptr).lbamode_top ==> HCI_ERROR_INPUT(result))
  && (!is_aligned_to_blt_entry_size_at_level(old_s, base, BLTWalk(old_s, base).level as int) ==> HCI_ERROR_BLT(result, BLTWalk(new_s, base).level as int))
  && (BLTAt(old_s, BLTWalk(old_s, base).blt, BLTWalk(old_s, base).level as int).lbamode != WorkerAt(old_s, worker_ptr).lbamode_value ==> HCI_ERROR_BLT(result, BLTWalk(new_s, base).level as int))
  && ((!(is_aligned_to_blt_entry_size_at_level(old_s, base, BLTWalk(old_s, base).level as int)) &&
       BLTAt(old_s, BLTWalk(old_s, base).blt, BLTWalk(old_s, base).level as int).lbamode == WorkerAt(old_s, worker_ptr).lbamode_value)
    ==> HCI_ERROR_BLT(result, BLTWalk(new_s, base).level as int))
  && (BLTAt(old_s, BLTWalk(old_s, base).blt, BLTWalk(old_s, base).level as int).lbamode == WorkerAt(old_s, worker_ptr).lbamode_value
    ==> HCI_ERROR_BLT(result, BLTWalk(new_s, base).level as int))
  && (any_part_of_range_from_base_to_top_is_live_in_auxiliary_blt(old_s, vd, base, top) ==> HCI_ERROR_BLT(result, BLTWalk(new_s, base).level as int))
  && (result == HCI_SUCCESS ==> WorkerAt(new_s, worker_ptr).lbamode_addr == min(top, top_of_address_range_covered_by_blt_reached_by_walk_before_command(new_s, base)))
  && (result == HCI_SUCCESS ==> out_top == min(top, top_of_address_range_covered_by_blt_reached_by_walk_before_command(new_s, base)))
  && ((!( (vd) % extent_size(old_s) != 0) &&
       is_enrollable_physical_address(old_s, vd) &&
       !(ExtentAt(old_s, vd).state != VD_STATE) &&
       !((worker_ptr) % extent_size(old_s) != 0) &&
       is_enrollable_physical_address(old_s, worker_ptr) &&
       !(ExtentAt(old_s, worker_ptr).state != WORKER_STATE) &&
       !(WorkerAt(old_s, worker_ptr).state == WORKER_RUNNING) &&
       WorkerOwnedByVault(old_s, worker_ptr, vd) &&
       !(top <= base) &&
       !(base != WorkerAt(old_s, worker_ptr).lbamode_addr) &&
       !(top > WorkerAt(old_s, worker_ptr).lbamode_top) &&
       is_aligned_to_blt_entry_size_at_level(old_s, base, BLTWalk(old_s, base).level as int) &&
       !(BLTAt(old_s, BLTWalk(old_s, base).blt, BLTWalk(old_s, base).level as int).lbamode != WorkerAt(old_s, worker_ptr).lbamode_value) &&
       !((!(is_aligned_to_blt_entry_size_at_level(old_s, base, BLTWalk(old_s, base).level as int)) &&
          BLTAt(old_s, BLTWalk(old_s, base).blt, BLTWalk(old_s, base).level as int).lbamode == WorkerAt(old_s, worker_ptr).lbamode_value)) &&
       !(BLTAt(old_s, BLTWalk(old_s, base).blt, BLTWalk(old_s, base).level as int).lbamode == WorkerAt(old_s, worker_ptr).lbamode_value) &&
       !(any_part_of_range_from_base_to_top_is_live_in_auxiliary_blt(old_s, vd, base, top)))
    ==> result == HCI_SUCCESS)
  && (result != HCI_SUCCESS
    ==> WorkerAt(new_s, worker_ptr).lbamode_addr == WorkerAt(old_s, worker_ptr).lbamode_addr)
  && (result != HCI_SUCCESS
    ==> out_top == 0)
  && (BLTAt(new_s, BLTWalk(new_s, base).blt, BLTWalk(new_s, base).level as int).lbamode == BLTAt(old_s, BLTWalk(old_s, base).blt, BLTWalk(old_s, base).level as int).lbamode)
}