(define-constant err-rate-failed (err u1000))
(define-constant err-test-failed (err u2000))

(define-public (test-rate (user-to-rate principal) (reputation-amount int))
  (let 
    (
      (user-to-rate-current-reputation (unwrap-panic (get-user-reputation-wd user-to-rate))))
      (if (or 
              (<= reputation-amount (if (>= (unwrap-panic (get-user-reputation-wd tx-sender)) 50) -20 -10))
              (>= reputation-amount (if (>= (unwrap-panic (get-user-reputation-wd tx-sender)) 50) 20 10))
              (<= (+ user-to-rate-current-reputation reputation-amount) -100)                 
              (>= (+ user-to-rate-current-reputation reputation-amount) 100)
              (is-eq user-to-rate tx-sender)
          )
        (ok false)
        (let ((user-to-rate-updated-reputation (unwrap-panic (get-user-reputation-wd user-to-rate)))) 
          (try! (rate-user user-to-rate reputation-amount))
          (asserts! (is-eq user-to-rate-updated-reputation (+ user-to-rate-current-reputation reputation-amount)) err-rate-failed)
          (ok true)
        )
      )
    )
)

;; (define-public (test-optional-decay-reputation (user principal))
;;   (let 
;;     (
;;       (initial-reputation (default-to 0 (get-user-reputation user)))
;;       (last-decay-time (default-to u0 (get last-decay (map-get? user-decay user))))
;;       (current-time stacks-block-height)
;;       (time-elapsed (- current-time last-decay-time))
;;     )
;;     (if (>= time-elapsed u1000)
;;       ;; Should apply decay
;;       (begin
;;         (let 
;;           (
;;             (decay-result (unwrap-panic (optional-decay-reputation user)))
;;             (new-reputation (default-to 0 (get-user-reputation user)))
;;           )
;;           (if (and (> initial-reputation 0) (< new-reputation initial-reputation))
;;             ;; Decay properly applied
;;             (ok true)
;;             ;; No decay was applied but it should have been
;;             (err err-test-failed))
;;         )
;;       )
;;       ;; Should not apply decay
;;       (begin
;;         (let 
;;           (
;;             (decay-result (unwrap-panic (optional-decay-reputation user)))
;;           )
;;           (if (is-eq decay-result initial-reputation)
;;             ;; No decay correctly applied
;;             (ok true)
;;             ;; Decay was wrongly applied
;;             (err err-test-failed))
;;         )
;;       )
;;     )
;;   )
;; )
