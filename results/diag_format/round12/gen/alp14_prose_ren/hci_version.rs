pub open spec fn hci_version_spec(req: Hciinterfaceversion, result: Hcicommandreturncode, lower: Hciinterfaceversion, higher: Hciinterfaceversion, old_s: S, new_s: S) -> bool {
  (result == HCI_SUCCESS ==> lower == req)
  && (result == HCI_SUCCESS ==> higher == higher)
  && ((!(result == HCI_SUCCESS)) ==> (result == HCI_ERROR_INPUT))
  && ((!(result == HCI_SUCCESS)) ==> (lower <= higher))
  && ((result != HCI_SUCCESS && result != HCI_ERROR_INPUT) ==> result == RSI_SUCCESS)
}