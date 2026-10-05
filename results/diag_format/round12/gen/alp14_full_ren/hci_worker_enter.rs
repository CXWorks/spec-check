pub open spec fn hci_worker_enter_spec(worker_ptr: UInt64, run_ptr: UInt64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  ((worker_ptr % extent_size(old_s) != 0) ==> result == RSI_ERROR_INPUT)
  && (ExtentAt(old_s, worker_ptr).enrollable == false ==> result == RSI_ERROR_INPUT)
  && (ExtentAt(old_s, worker_ptr).state != WORKER ==> result == RSI_ERROR_INPUT)
  && (ExtentAt(old_s, run_ptr).accessible_in_non_secure_dpas == false ==> result == RSI_ERROR_INPUT)
  && ((run_ptr % extent_size(old_s) != 0) ==> result == RSI_ERROR_INPUT)
  && (ExtentAt(old_s, worker_ptr).state == WORKER_RUNNING ==> result == RSI_ERROR_WORKER)
  && (ExtentAt(old_s, worker_ptr).not_runnable == true ==> result == RSI_ERROR_WORKER)
  && ((ExtentAt(old_s, run_ptr).emul_mmio == HCI_EMULATED_MMIO) && (ExtentAt(old_s, worker_ptr).emulatable_abort != EMULATABLE_ABORT) ==> result == RSI_ERROR_WORKER)
  && (VaultThatOwnsWorker(old_s, worker_ptr).state == VAULT_NEW ==> result == RSI_ERROR_VAULT(0))
  && (VaultThatOwnsWorker(old_s, worker_ptr).state == VAULT_SYSTEM_OFF ==> result == RSI_ERROR_VAULT(1))
  && (ExtentAt(old_s, worker_ptr).pending != WORKER_PENDING_NONE ==> result == RSI_ERROR_WORKER)
  && (result == RSI_SUCCESS ==> ExtentAt(new_s, worker_ptr).emulatable_abort == ExtentAt(new_s, worker_ptr).emulatable_abort + 1)
  && ((!( (worker_ptr % extent_size(old_s) != 0) ) &&
       !( ExtentAt(old_s, worker_ptr).enrollable == false ) &&
       !( ExtentAt(old_s, worker_ptr).state != WORKER ) &&
       !( ExtentAt(old_s, run_ptr).accessible_in_non_secure_dpas == false ) &&
       !( (run_ptr % extent_size(old_s) != 0) ) &&
       !( ExtentAt(old_s, worker_ptr).state == WORKER_RUNNING ) &&
       !( ExtentAt(old_s, worker_ptr).not_runnable == true ) &&
       !((ExtentAt(old_s, run_ptr).emul_mmio == HCI_EMULATED_MMIO) && (ExtentAt(old_s, worker_ptr).emulatable_abort != EMULATABLE_ABORT)) &&
       !( VaultThatOwnsWorker(old_s, worker_ptr).state == VAULT_NEW ) &&
       !( VaultThatOwnsWorker(old_s, worker_ptr).state == VAULT_SYSTEM_OFF ) &&
       !( ExtentAt(old_s, worker_ptr).pending != WORKER_PENDING_NONE ))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> ExtentAt(new_s, worker_ptr).emulatable_abort == ExtentAt(old_s, worker_ptr).emulatable_abort)
}