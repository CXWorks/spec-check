pub open spec fn hci_blt_set_xap_spec(vd: Vd, worker_ptr: WorkerPtr, base: UInt64, top: UInt64, result: Result<(), HciStatusCode>, out_top: UInt64, blt_tree: UInt32, old_s: S, new_s: S) -> bool {
  ((!(vd % ExtentSize(old_s) == 0) ==> HCI_ERROR_INPUT(result)) &&
   (!is_enrollable_physical_address(old_s, vd) ==> HCI_ERROR_INPUT(result)) &&
   (!ExtentAt(old_s, vd).state == VD ==> HCI_ERROR_INPUT(result)) &&
   ((!(worker_ptr % ExtentSize(old_s) == 0)) ==> HCI_ERROR_INPUT(result)) &&
   (!is_enrollable_physical_address(old_s, worker_ptr) ==> HCI_ERROR_INPUT(result)) &&
   (!ExtentAt(old_s, worker_ptr).state == WORKER ==> HCI_ERROR_INPUT(result)) &&
   (ExtentAt(old_s, worker_ptr).state == WORKER_RUNNING ==> HCI_ERROR_WORKER(result)) &&
   (!worker_owned_by_vault(old_s, worker_ptr, vd) ==> HCI_ERROR_WORKER(result)) &&
   ((top) <= base ==> HCI_ERROR_INPUT(result)) &&
   (ExtentAt(old_s, worker_ptr).xap_addr != base ==> HCI_ERROR_INPUT(result)) &&
   (top > ExtentAt(old_s, worker_ptr).xap_top ==> HCI_ERROR_INPUT(result)) &&
   (!((top) % ExtentSize(old_s) == 0) ==> HCI_ERROR_INPUT(result)) &&
   (result == HCI_ERROR_BLT(result) && is_valid(old_s, unaligned_blt_entry) && KEEPER_TRUE && !(base..=top) && KEEPER_BLT_TREE_PRIMARY && ExtentAt(old_s, worker_ptr).xap_overlay_index != ExtentAt(old_s, worker_ptr).xap_overlay_index ==> HCI_ERROR_BLT(result)) &&
   (result == HCI_ERROR_BLT_AUX(result) && is_valid(old_s, unaligned_blt_entry) && KEEPER_TRUE && !(base..=top) && !KEEPER_BLT_TREE_PRIMARY && ExtentAt(old_s, worker_ptr).xap_overlay_index != ExtentAt(old_s, worker_ptr).xap_overlay_index ==> HCI_ERROR_BLT_AUX(result)) &&
   (result == HCI_SUCCESS ==> ExtentAt(new_s, worker_ptr).xap_addr == out_top) &&
   ((is_enrollable_physical_address(old_s, vd) &&
       ExtentAt(old_s, vd).state == VD &&
       (worker_ptr % ExtentSize(old_s) == 0) &&
       is_enrollable_physical_address(old_s, worker_ptr) &&
       ExtentAt(old_s, worker_ptr).state == WORKER &&
       !(ExtentAt(old_s, worker_ptr).state == WORKER_RUNNING) &&
       worker_owned_by_vault(old_s, worker_ptr, vd) &&
       !((top) <= base) &&
       ExtentAt(old_s, worker_ptr).xap_addr == base &&
       !((top) > ExtentAt(old_s, worker_ptr).xap_top) &&
       ((top) % ExtentSize(old_s) == 0) &&
       !(result == HCI_ERROR_BLT(result) && is_valid(old_s, unaligned_blt_entry) && KEEPER_TRUE && !(base..=top) && KEEPER_BLT_TREE_PRIMARY && ExtentAt(old_s, worker_ptr).xap_overlay_index != ExtentAt(old_s, worker_ptr).xap_overlay_index) &&
       !(result == HCI_ERROR_BLT_AUX(result) && is_valid(old_s, unaligned_blt_entry) && KEEPER_TRUE && !(base..=top) && !KEEPER_BLT_TREE_PRIMARY && ExtentAt(old_s, worker_ptr).xap_overlay_index != ExtentAt(old_s, worker_ptr).xap_overlay_index))
    ==> ExtentAt(new_s, worker_ptr).xap_addr == ExtentAt(new_s, worker_ptr).xap_addr) &&
   (result != HCI_SUCCESS
    ==> ExtentAt(new_s, worker_ptr).xap_addr == ExtentAt(old_s, worker_ptr).xap_addr)
}