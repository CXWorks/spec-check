pub open spec fn sbi_debug_disable_triggers_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (
        (exists (trig_idx: UInt64),
         trig_idx < (trig_idx_base as int) + (trig_idx_mask as int) &&
         (trig_idx - trig_idx_base as int) < (trig_idx_mask as int) &&
         (trig_idx - trig_idx_base as int) >= 0 &&
         (trig_idx >= (trig_idx_base as int) &&
          (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
          (trig_idx >= (trig_idx_base as int) &&
           (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
           (trig_idx >= (trig_idx_base as int) &&
            (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
            (trig_idx >= (trig_idx_base as int) &&
             (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
             (trig_idx >= (trig_idx_base as int) &&
              (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
              (trig_idx >= (trig_idx_base as int) &&
               (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
               (trig_idx >= (trig_idx_base as int) &&
                (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                (trig_idx >= (trig_idx_base as int) &&
                 (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                 (trig_idx >= (trig_idx_base as int) &&
                  (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                  (trig_idx >= (trig_idx_base as int) &&
                   (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                   (trig_idx >= (trig_idx_base as int) &&
                    (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                    (trig_idx >= (trig_idx_base as int) &&
                     (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                     (trig_idx >= (trig_idx_base as int) &&
                      (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                      (trig_idx >= (trig_idx_base as int) &&
                       (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                       (trig_idx >= (trig_idx_base as int) &&
                        (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                        (trig_idx >= (trig_idx_base as int) &&
                         (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                         (trig_idx >= (trig_idx_base as int) &&
                          (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                          (trig_idx >= (trig_idx_base as int) &&
                           (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                           (trig_idx >= (trig_idx_base as int) &&
                            (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                            (trig_idx >= (trig_idx_base as int) &&
                             (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                             (trig_idx >= (trig_idx_base as int) &&
                              (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                              (trig_idx >= (trig_idx_base as int) &&
                               (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                               (trig_idx >= (trig_idx_base as int) &&
                                (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                (trig_idx >= (trig_idx_base as int) &&
                                 (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                 (trig_idx >= (trig_idx_base as int) &&
                                  (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                  (trig_idx >= (trig_idx_base as int) &&
                                   (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                   (trig_idx >= (trig_idx_base as int) &&
                                    (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                    (trig_idx >= (trig_idx_base as int) &&
                                     (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                     (trig_idx >= (trig_idx_base as int) &&
                                      (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                      (trig_idx >= (trig_idx_base as int) &&
                                       (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                       (trig_idx >= (trig_idx_base as int) &&
                                        (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                        (trig_idx >= (trig_idx_base as int) &&
                                         (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                         (trig_idx >= (trig_idx_base as int) &&
                                          (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                          (trig_idx >= (trig_idx_base as int) &&
                                           (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                           (trig_idx >= (trig_idx_base as int) &&
                                            (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                            (trig_idx >= (trig_idx_base as int) &&
                                             (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                             (trig_idx >= (trig_idx_base as int) &&
                                              (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                              (trig_idx >= (trig_idx_base as int) &&
                                               (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                               (trig_idx >= (trig_idx_base as int) &&
                                                (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                (trig_idx >= (trig_idx_base as int) &&
                                                 (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                 (trig_idx >= (trig_idx_base as int) &&
                                                  (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                  (trig_idx >= (trig_idx_base as int) &&
                                                   (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                   (trig_idx >= (trig_idx_base as int) &&
                                                    (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                    (trig_idx >= (trig_idx_base as int) &&
                                                     (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                     (trig_idx >= (trig_idx_base as int) &&
                                                      (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                      (trig_idx >= (trig_idx_base as int) &&
                                                       (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                       (trig_idx >= (trig_idx_base as int) &&
                                                        (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                        (trig_idx >= (trig_idx_base as int) &&
                                                         (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                         (trig_idx >= (trig_idx_base as int) &&
                                                          (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                          (trig_idx >= (trig_idx_base as int) &&
                                                           (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                           (trig_idx >= (trig_idx_base as int) &&
                                                            (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                            (trig_idx >= (trig_idx_base as int) &&
                                                             (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                             (trig_idx >= (trig_idx_base as int) &&
                                                              (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                              (trig_idx >= (trig_idx_base as int) &&
                                                               (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                               (trig_idx >= (trig_idx_base as int) &&
                                                                (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                (trig_idx >= (trig_idx_base as int) &&
                                                                 (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                 (trig_idx >= (trig_idx_base as int) &&
                                                                  (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                  (trig_idx >= (trig_idx_base as int) &&
                                                                   (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                   (trig_idx >= (trig_idx_base as int) &&
                                                                    (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                    (trig_idx >= (trig_idx_base as int) &&
                                                                     (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                     (trig_idx >= (trig_idx_base as int) &&
                                                                      (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                      (trig_idx >= (trig_idx_base as int) &&
                                                                       (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                       (trig_idx >= (trig_idx_base as int) &&
                                                                        (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                        (trig_idx >= (trig_idx_base as int) &&
                                                                         (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                         (trig_idx >= (trig_idx_base as int) &&
                                                                          (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                          (trig_idx >= (trig_idx_base as int) &&
                                                                           (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                           (trig_idx >= (trig_idx_base as int) &&
                                                                            (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                            (trig_idx >= (trig_idx_base as int) &&
                                                                             (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                             (trig_idx >= (trig_idx_base as int) &&
                                                                              (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                              (trig_idx >= (trig_idx_base as int) &&
                                                                               (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                               (trig_idx >= (trig_idx_base as int) &&
                                                                                (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                (trig_idx >= (trig_idx_base as int) &&
                                                                                 (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                 (trig_idx >= (trig_idx_base as int) &&
                                                                                  (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                  (trig_idx >= (trig_idx_base as int) &&
                                                                                   (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                   (trig_idx >= (trig_idx_base as int) &&
                                                                                    (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                    (trig_idx >= (trig_idx_base as int) &&
                                                                                     (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                     (trig_idx >= (trig_idx_base as int) &&
                                                                                      (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                      (trig_idx >= (trig_idx_base as int) &&
                                                                                       (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                       (trig_idx >= (trig_idx_base as int) &&
                                                                                        (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                        (trig_idx >= (trig_idx_base as int) &&
                                                                                         (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                         (trig_idx >= (trig_idx_base as int) &&
                                                                                          (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                          (trig_idx >= (trig_idx_base as int) &&
                                                                                           (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                           (trig_idx >= (trig_idx_base as int) &&
                                                                                            (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                            (trig_idx >= (trig_idx_base as int) &&
                                                                                             (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                             (trig_idx >= (trig_idx_base as int) &&
                                                                                              (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                              (trig_idx >= (trig_idx_base as int) &&
                                                                                               (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                               (trig_idx >= (trig_idx_base as int) &&
                                                                                                (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                (trig_idx >= (trig_idx_base as int) &&
                                                                                                 (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                 (trig_idx >= (trig_idx_base as int) &&
                                                                                                  (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                  (trig_idx >= (trig_idx_base as int) &&
                                                                                                   (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                   (trig_idx >= (trig_idx_base as int) &&
                                                                                                    (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                    (trig_idx >= (trig_idx_base as int) &&
                                                                                                     (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                     (trig_idx >= (trig_idx_base as int) &&
                                                                                                      (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                      (trig_idx >= (trig_idx_base as int) &&
                                                                                                       (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                       (trig_idx >= (trig_idx_base as int) &&
                                                                                                        (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                        (trig_idx >= (trig_idx_base as int) &&
                                                                                                         (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                         (trig_idx >= (trig_idx_base as int) &&
                                                                                                          (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                          (trig_idx >= (trig_idx_base as int) &&
                                                                                                           (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                           (trig_idx >= (trig_idx_base as int) &&
                                                                                                            (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                            (trig_idx >= (trig_idx_base as int) &&
                                                                                                             (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                             (trig_idx >= (trig_idx_base as int) &&
                                                                                                              (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                              (trig_idx >= (trig_idx_base as int) &&
                                                                                                               (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                               (trig_idx >= (trig_idx_base as int) &&
                                                                                                                (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                (trig_idx >= (trig_idx_base as int) &&
                                                                                                                 (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                 (trig_idx >= (trig_idx_base as int) &&
                                                                                                                  (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                  (trig_idx >= (trig_idx_base as int) &&
                                                                                                                   (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                   (trig_idx >= (trig_idx_base as int) &&
                                                                                                                    (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                    (trig_idx >= (trig_idx_base as int) &&
                                                                                                                     (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                     (trig_idx >= (trig_idx_base as int) &&
                                                                                                                      (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                      (trig_idx >= (trig_idx_base as int) &&
                                                                                                                       (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                       (trig_idx >= (trig_idx_base as int) &&
                                                                                                                        (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                        (trig_idx >= (trig_idx_base as int) &&
                                                                                                                         (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                         (trig_idx >= (trig_idx_base as int) &&
                                                                                                                          (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                          (trig_idx >= (trig_idx_base as int) &&
                                                                                                                           (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                           (trig_idx >= (trig_idx_base as int) &&
                                                                                                                            (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                            (trig_idx >= (trig_idx_base as int) &&
                                                                                                                             (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                             (trig_idx >= (trig_idx_base as int) &&
                                                                                                                              (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                              (trig_idx >= (trig_idx_base as int) &&
                                                                                                                               (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                               (trig_idx >= (trig_idx_base as int) &&
                                                                                                                                (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                                (trig_idx >= (trig_idx_base as int) &&
                                                                                                                                 (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                                 (trig_idx >= (trig_idx_base as int) &&
                                                                                                                                  (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                                  (trig_idx >= (trig_idx_base as int) &&
                                                                                                                                   (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                                   (trig_idx >= (trig_idx_base as int) &&
                                                                                                                                    (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                                    (trig_idx >= (trig_idx_base as int) &&
                                                                                                                                     (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                                     (trig_idx >= (trig_idx_base as int) &&
                                                                                                                                      (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                                      (trig_idx >= (trig_idx_base as int) &&
                                                                                                                                       (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                                       (trig_idx >= (trig_idx_base as int) &&
                                                                                                                                        (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                                        (trig_idx >= (trig_idx_base as int) &&
                                                                                                                                         (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                                         (trig_idx >= (trig_idx_base as int) &&
                                                                                                                                          (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                                          (trig_idx >= (trig_idx_base as int) &&
                                                                                                                                           (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                                           (trig_idx >= (trig_idx_base as int) &&
                                                                                                                                            (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                                            (trig_idx >= (trig_idx_base as int) &&
                                                                                                                                             (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                                             (trig_idx >= (trig_idx_base as int) &&
                                                                                                                                              (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                                              (trig_idx >= (trig_idx_base as int) &&
                                                                                                                                               (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                                               (trig_idx >= (trig_idx_base as int) &&
                                                                                                                                                (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                                                (trig_idx >= (trig_idx_base as int) &&
                                                                                                                                                 (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                                                 (trig_idx >= (trig_idx_base as int) &&
                                                                                                                                                  (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                                                                  (trig_idx >= (trig_idx_base as int) &&
                                                                                                                                                   (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                       (trig_idx >= (trig_idx_base as int) &&
                                                                                                        (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                        (trig_idx >= (trig_idx_base as int) &&
                                                                                                         (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                         (trig_idx >= (trig_idx_base as int) &&
                                                                                                          (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                          (trig_idx >= (trig_idx_base as int) &&
                                                                                                           (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                           (trig_idx >= (trig_idx_base as int) &&
                                                                                                            (trig_idx < (trig_idx_base as int) + (trig_idx_mask as int)) &&
                                                                                                            (trig_idx >= (trig_idx_base as int) &&
                                                                                                             (trig_idx < (trig_idx_base as