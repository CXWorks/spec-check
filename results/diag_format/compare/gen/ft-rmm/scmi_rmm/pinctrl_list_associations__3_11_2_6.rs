pub open spec fn pinctrl_list_associations__3_11_2_6_spec(identifier: UInt32, flags: PinctrlListAssociationsFlags, index: UInt32, status: Int32, flags1: PinctrlListAssociationsFlags1, array: [UInt16; 16], old_s: S, new_s: S) -> bool {
  (!GroupOrFunctionExists(old_s, identifier, flags.selector) ==> ResultEqual(status, NOT_FOUND))
  && (!IsRequestSupported(old_s, identifier, flags, index) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentMayListAssociations(old_s, calling_agent, identifier, flags.selector) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> flags1.num_returned == 0)
  && (ResultEqual(status, SUCCESS) ==> flags1.remaining == NumRemainingAssociations(new_s, identifier, flags.selector, index, 0))
  && (ResultEqual(status, SUCCESS) ==> array[0..0] == AssociatedIdentifiers(new_s, identifier, flags.selector)[index .. index+0])
  && (ResultEqual(status, SUCCESS) ==> IsAscending(new_s, array[0..0]))
  && (ResultEqual(status, SUCCESS) ==> flags1.reserved == 0)
  && ((GroupOrFunctionExists(old_s, identifier, flags.selector) &&
       IsRequestSupported(old_s, identifier, flags, index) &&
       AgentMayListAssociations(old_s, calling_agent, identifier, flags.selector))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> flags1.num_returned == 0)
  && (result != SUCCESS
    ==> flags1.remaining == NumRemainingAssociations(new_s, identifier, flags.selector, index, 0))
  && (result != SUCCESS
    ==> flags1.reserved == 0)
}