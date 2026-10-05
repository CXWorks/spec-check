pub open spec fn hci_worker_enter_spec(worker_ptr: Address, run_ptr: Address, result: Hcicommandreturncode, old_s: S, new_s: S) -> bool {
  (Workerat(old_s, worker_ptr).state == WORKER ==> result == HCI_ERROR_WORKER)
  && (Workerat(old_s, worker_ptr).not_runnable ==> result == HCI_ERROR_WORKER)
  && (Workerat(old_s, worker_ptr).emul_mmio == HCI_EMULATED_MMIO && Workerat(old_s, worker_ptr).emulatable_abort != EMULATABLE_ABORT ==> result == HCI_ERROR_WORKER)
  && (Vaultat(old_s, Workerat(old_s, worker_ptr).owner).state == VAULT_NEW ==> result == HCI_ERROR_VAULT(0))
  && (Vaultat(old_s, Workerat(old_s, worker_ptr).owner).state == VAULT_SYSTEM_OFF ==> result == HCI_ERROR_VAULT(1))
  && (Workerat(old_s, worker_ptr).pending != WORKER_PENDING_NONE ==> result == HCI_ERROR_WORKER)
  && (result == HCI_SUCCESS ==> Workerat(new_s, worker_ptr).emulatable_abort == Workerat(new_s, worker_ptr).emulatable_abort + 1)
  && ((!(Workerat(old_s, worker_ptr).state == WORKER) &&
       Workerat(old_s, worker_ptr).not_runnable == false &&
       !(Workerat(old_s, worker_ptr).emul_mmio == HCI_EMULATED_MMIO && Workerat(old_s, worker_ptr).emulatable_abort != EMULATABLE_ABORT) &&
       !(Vaultat(old_s, Workerat(old_s, worker_ptr).owner).state == VAULT_NEW) &&
       !(Vaultat(old_s, Workerat(old_s, worker_ptr).owner).state == VAULT_SYSTEM_OFF) &&
       Workerat(old_s, worker_ptr).pending == WORKER_PENDING_NONE)
    ==> result == HCI_SUCCESS)
  && (result != HCI_SUCCESS
    ==> Workerat(new_s, worker_ptr).emulatable_abort == Workerat(old_s, worker_ptr).emulatable_abort)
}