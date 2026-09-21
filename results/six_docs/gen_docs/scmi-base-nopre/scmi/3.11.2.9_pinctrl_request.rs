pub open spec fn pinctrl_request_spec(result: i32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (new_s.pin_controlled.contains(old_s.identifier) && old_s.pin_controlled.contains(old_s.identifier)))
    && (result == -1 ==> !old_s.pin_controlled.contains(old_s.identifier))
    && (result == -2 ==> (old_s.identifier < 0 || old_s.flags != 0))
    && (result == -3 ==> !old_s.agent_allowed(old_s.identifier))
    && (result == -4 ==> old_s.pin_controlled.contains(old_s.identifier))
}