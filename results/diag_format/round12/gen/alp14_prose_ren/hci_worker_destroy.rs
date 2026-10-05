pub open spec fn hci_worker_destroy_spec(worker_ptr: Address, result: Hcicommandreturncode, old_s: S, new_s: S) -> bool {
  ((!(Workerat(old_s, worker_ptr).state == ENROLLED) || (Workerat(old_s, worker_ptr).state == WORKER)) ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_WORKER ==> Workerat(new_s, worker_ptr).state == WORKER_RUNNING)
  && (result == HCI_SUCCESS ==> Workerat(new_s, worker_ptr).state == ENROLLED)
  && (result == HCI_SUCCESS ==> Vaultat(new_s, Vaultat(new_s, Workerat(new_s, worker_ptr).owner)).num_workers == Vaultat(new_s, Workerat(new_s, worker_ptr).owner).num_workers - 1)
  && ((!(Workerat(old_s, worker_ptr).state == ENROLLED) &&
       Workerat(old_s, worker_ptr).state != WORKER)
    ==> result == HCI_SUCCESS)
  && (result != HCI_SUCCESS
    ==> Workerat(new_s, worker_ptr).state == Workerat(old_s, worker_ptr).state)
  && (result != HCI_SUCCESS
    ==> Vaultat(new_s, Vaultat(new_s, Workerat(new_s, worker_ptr).owner)).num_workers == Vaultat(old_s, Workerat(old_s, worker_ptr).owner).num_workers)
  && (result == HCI_SUCCESS
    ==> Vaultat(new_s, Vaultat(new_s, Workerat(new_s, worker_ptr).owner)).num_workers == Vaultat(old_s, Workerat(old_s, worker_ptr).owner).num_workers - 1)
}