pub open spec fn pinctrl_list_associations__3_11_2_6_spec(result: Int32, flags: PinControlFlags, array: [UInt16], old_s: S, new_s: S) -> bool {
    (!GroupOrFunctionExists(identifier(old_s), flags.selector) ==> ResultEqual(result, NOT_FOUND))
    && (!IsRequestSupported(identifier(old_s), flags, index(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentMayListAssociations(calling_agent(old_s), identifier(old_s), flags.selector) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (flags.num_returned == N && flags.remaining == NumRemainingAssociations(identifier(old_s), flags.selector, index(old_s), N) && array[0..N-1] == AssociatedIdentifiers(identifier(old_s), flags.selector)[index(old_s)..index(old_s)+N-1] && IsAscending(array[0..N-1]) && flags.reserved == 0))
}