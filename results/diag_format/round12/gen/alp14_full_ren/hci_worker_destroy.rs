pub open spec fn hci_worker_destroy_spec(worker_ptr: UInt64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  ((worker_ptr % extent_size(old_s) != 0) ==> result == RSI_ERROR_INPUT)
  && (!can_be_enrolled(old_s, worker_ptr) ==> result == RSI_ERROR_INPUT)
  && (ExtentAt(old_s, worker_ptr).state != WORKER ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS && ExtentAt(old_s, worker_ptr).state == WORKER_RUNNING ==> result == RSI_ERROR_WORKER)
  && (result == RSI_SUCCESS ==> ExtentAt(new_s, worker_ptr).state == ENROLLED)
  && (result == RSI_SUCCESS && ExtentAt(old_s, worker_ptr).state == WORKER ==> ExtentAt(new_s, worker_ptr).state == ENROLLED)
  && (result == RSI_SUCCESS && ExtentAt(old_s, worker_ptr).state != WORKER ==> ExtentAt(new_s, worker_ptr).state == ExtentAt(old_s, worker_ptr).state)
  && (result == RSI_SUCCESS ==> VaultAt(new_s, VaultAt(old_s, worker_ptr).vd_addr).num_workers == VaultAt(old_s, VaultAt(old_s, worker_ptr).vd_addr).num_workers - 1)
  && ((!(worker_ptr % extent_size(old_s) != 0) &&
       can_be_enrolled(old_s, worker_ptr) &&
       !(ExtentAt(old_s, worker_ptr).state != WORKER) &&
       !(result == RSI_SUCCESS && ExtentAt(old_s, worker_ptr).state == WORKER_RUNNING))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> ExtentAt(new_s, worker_ptr).state == ExtentAt(old_s, worker_ptr).state)
  && (result != RSI_SUCCESS
    ==> VaultAt(new_s, VaultAt(old_s, worker_ptr).vd_addr).num_workers == VaultAt(old_s, VaultAt(old_s, worker_ptr).vd_addr).num_workers)
  && (result == RSI_ERROR_INPUT || result == RSI_ERROR_WORKER || result == RSI_ERROR_STATE || result == RSI_INCOMPLETE || result == RSI_ERROR_UNKNOWN
    ==> ExtentAt(new_s, worker_ptr).state == ExtentAt(old_s, worker_ptr).state)
}