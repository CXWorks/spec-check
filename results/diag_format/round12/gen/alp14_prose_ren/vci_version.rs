pub open spec fn vci_version_spec(req: Vciinterfaceversion, result: Vcicommandreturncode, lower: Vciinterfaceversion, higher: Vciinterfaceversion, old_s: S, new_s: S) -> bool {
  (result == VCI_SUCCESS ==> lower == req)
  && (result == VCI_SUCCESS ==> higher == higher)
  && ((!(result == VCI_SUCCESS)) ==> lower == lower)
  && ((!(result == VCI_SUCCESS)) ==> higher == higher)
  && ((result == VCI_ERROR_INPUT) ==> lower <= higher)
  && ((result == VCI_ERROR_INPUT) ==> lower <= req)
  && (result != VCI_SUCCESS && result != VCI_ERROR_INPUT ==> lower == lower)
  && (result != VCI_SUCCESS && result != VCI_ERROR_INPUT ==> higher == higher)
}