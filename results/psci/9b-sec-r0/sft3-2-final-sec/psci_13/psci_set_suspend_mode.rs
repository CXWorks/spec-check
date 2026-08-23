pub open spec fn psci_set_suspend_mode_spec(mode: int, result: int, old_s: S, new_s: S) -> bool {
  (result == PSCI_INVALID_PARAMETERS ==> (mode != 0 && mode != 1))
  && ((!( (CpuIsOn(old_s, 0) || CpuIsOnPending(old_s, 0)) &&
         !( (CpuIsOn(old_s, 1) || CpuIsOnPending(old_s, 1)) &&
            !( (CpuIsOn(old_s, 2) || CpuIsOnPending(old_s, 2)) &&
               !( (CpuIsOn(old_s, 3) || CpuIsOnPending(old_s, 3)) &&
                  !( (CpuIsOn(old_s, 4) || CpuIsOnPending(old_s, 4)) &&
                     !( (CpuIsOn(old_s, 5) || CpuIsOnPending(old_s, 5)) &&
                        !( (CpuIsOn(old_s, 6) || CpuIsOnPending(old_s, 6)) &&
                           !( (CpuIsOn(old_s, 7) || CpuIsOnPending(old_s, 7)) &&
                              !( (CpuIsOn(old_s, 8) || CpuIsOnPending(old_s, 8)) &&
                                 !( (CpuIsOn(old_s, 9) || CpuIsOnPending(old_s, 9)) &&
                                    !( (CpuIsOn(old_s, 10) || CpuIsOnPending(old_s, 10)) &&
                                       !( (CpuIsOn(old_s, 11) || CpuIsOnPending(old_s, 11)) &&
                                          !( (CpuIsOn(old_s, 12) || CpuIsOnPending(old_s, 12)) &&
                                             !( (CpuIsOn(old_s, 13) || CpuIsOnPending(old_s, 13)) &&
                                                !( (CpuIsOn(old_s, 14) || CpuIsOnPending(old_s, 14)) &&
                                                   !( (CpuIsOn(old_s, 15) || CpuIsOnPending(old_s, 15)) &&
                                                      !( (CpuIsOn(old_s, 16) || CpuIsOnPending(old_s, 16)) &&
                                                         !( (CpuIsOn(old_s, 17) || CpuIsOnPending(old_s, 17)) &&
                                                            !( (CpuIsOn(old_s, 18) || CpuIsOnPending(old_s, 18)) &&
                                                               !( (CpuIsOn(old_s, 19) || CpuIsOnPending(old_s, 19)) &&
                                                                  !( (CpuIsOn(old_s, 20) || CpuIsOnPending(old_s, 20)) &&
                                                                     !( (CpuIsOn(old_s, 21) || CpuIsOnPending(old_s, 21)) &&
                                                                        !( (CpuIsOn(old_s, 22) || CpuIsOnPending(old_s, 22)) &&
                                                                           !( (CpuIsOn(old_s, 23) || CpuIsOnPending(old_s, 23)) &&
                                                                              !( (CpuIsOn(old_s, 24) || CpuIsOnPending(old_s, 24)) &&
                                                                                 !( (CpuIsOn(old_s, 25) || CpuIsOnPending(old_s, 25)) &&
                                                                                    !( (CpuIsOn(old_s, 26) || CpuIsOnPending(old_s, 26)) &&
                                                                                       !( (CpuIsOn(old_s, 27) || CpuIsOnPending(old_s, 27)) &&
                                                                                          !( (CpuIsOn(old_s, 28) || CpuIsOnPending(old_s, 28)) &&
                                                                                             !( (CpuIsOn(old_s, 29) || CpuIsOnPending(old_s, 29)) &&
                                                                                                !( (CpuIsOn(old_s, 30) || CpuIsOnPending(old_s, 30)) &&
                                                                                                   !( (CpuIsOn(old_s, 31) || CpuIsOnPending(old_s, 31)) &&
                                                                                                      !( (CpuIsOn(old_s, 32) || CpuIsOnPending(old_s, 32)) &&
                                                                                                         !( (CpuIsOn(old_s, 33) || CpuIsOnPending(old_s, 33)) &&
                                                                                                            !( (CpuIsOn(old_s, 34) || CpuIsOnPending(old_s, 34)) &&
                                                                                                               !( (CpuIsOn(old_s, 35) || CpuIsOnPending(old_s, 35)) &&
                                                                                                                  !( (CpuIsOn(old_s, 36) || CpuIsOnPending(old_s, 36)) &&
                                                                                                                     !( (CpuIsOn(old_s, 37) || CpuIsOnPending(old_s, 37)) &&
                                                                                                                        !( (CpuIsOn(old_s, 38) || CpuIsOnPending(old_s, 38)) &&
                                                                                                                           !( (CpuIsOn(old_s, 39) || CpuIsOnPending(old_s, 39)) &&
                                                                                                                              !( (CpuIsOn(old_s, 40) || CpuIsOnPending(old_s, 40)) &&
                                                                                                                                 !( (CpuIsOn(old_s, 41) || CpuIsOnPending(old_s, 41)) &&
                                                                                                                                    !( (CpuIsOn(old_s, 42) || CpuIsOnPending(old_s, 42)) &&
                                                                                                                                       !( (CpuIsOn(old_s, 43) || CpuIsOnPending(old_s, 43)) &&
                                                                                                                                          !( (CpuIsOn(old_s, 44) || CpuIsOnPending(old_s, 44)) &&
                                                                                                                                                 !( (CpuIsOn(old_s, 45) || CpuIsOnPending(old_s, 45)) &&
                                                                                                                                                    !( (CpuIsOn(old_s, 46) || CpuIsOnPending(old_s, 46)) &&
                                                                                                                                                       !( (CpuIsOn(old_s, 47) || CpuIsOnPending(old_s, 47)) &&
                                                                                                                                                          !( (CpuIsOn(old_s, 48) || CpuIsOnPending(old_s, 48)) &&
                                                                                                                                                             !( (CpuIsOn(old_s, 49) || CpuIsOnPending(old_s, 49)) &&
                                                                                                                                                                !( (CpuIsOn(old_s, 50) || CpuIsOnPending(old_s, 50)) &&
                                                                                                                                                                   !( (CpuIsOn(old_s, 51) || CpuIsOnPending(old_s, 51)) &&
                                                                                                                                                                      !( (CpuIsOn(old_s, 52) || CpuIsOnPending(old_s, 52)) &&
                                                                                                                                                                         !( (CpuIsOn(old_s, 53) || CpuIsOnPending(old_s, 53)) &&
                                                                                                                                                                            !( (CpuIsOn(old_s, 54) || CpuIsOnPending(old_s, 54)) &&
                                                                                                                                                                               !( (CpuIsOn(old_s, 55) || CpuIsOnPending(old_s, 55)) &&
                                                                                                                                                                                  !( (CpuIsOn(old_s, 56) || CpuIsOnPending(old_s, 56)) &&
                                                                                                                                                                                     !( (CpuIsOn(old_s, 57) || CpuIsOnPending(old_s, 57)) &&
                                                                                                                                                                                        !( (CpuIsOn(old_s, 58) || CpuIsOnPending(old_s, 58)) &&
                                                                                                                                                                                           !( (CpuIsOn(old_s, 59) || CpuIsOnPending(old_s, 59)) &&
                                                                                                                                                                                              !( (CpuIsOn(old_s, 60) || CpuIsOnPending(old_s, 60)) &&
                                                                                                                                                                                                 !( (CpuIsOn(old_s, 61) || CpuIsOnPending(old_s, 61)) &&
                                                                                                                                                                                                    !( (CpuIsOn(old_s, 62) || CpuIsOnPending(old_s, 62)) &&
                                                                                                                                                                                                       !( (CpuIsOn(old_s, 63) || CpuIsOnPending(old_s, 63))
                                                                                                                                                                                                    )
                                                                                                                                                       )
                                                                                                                                                  )
                                                                                                                                             )
                                                                                            )
                                                                                       )
                                                                                  )
                                                                             )
                                                                        )
                                                                   )
                                                              )
                                                         )
                                                    )
                                               )
                                          )
                                     )
                                 )
                              )
                           )
                        )
                     )
                  )
               )
            )
         )
      ))
    ==> result == PSCI_SUCCESS)
  && ((result != PSCI_SUCCESS)
    ==> cpu_state(new_s) == cpu_state(old_s))
}