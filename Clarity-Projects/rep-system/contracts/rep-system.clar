(define-constant err-user-inexistent (err u100))
(define-constant err-user-already-existent (err u101))
(define-constant err-same-user (err u102))
(define-constant err-invalid-reputation-amount (err u103))
(define-constant err-user-to-rate-inexistent (err u104))
(define-constant err-owner-only (err u105))

(define-constant contract-owner tx-sender)

(define-map user-reputation principal {reputation: int})

(define-public (enroll)
  (begin
    (asserts! (is-eq contract-caller contract-owner) err-owner-only)
    (asserts! (is-none (map-get? user-reputation tx-sender)) err-user-already-existent)
    (map-set user-reputation tx-sender {reputation: 0})
    (ok true)
  )
)

(define-public (rate-user (user-to-rate principal) (reputation-amount int))
  (begin 
    (asserts! (is-eq contract-caller contract-owner) err-owner-only)
    (asserts! (is-some (map-get? user-reputation tx-sender)) err-user-inexistent)
    (asserts! (is-some (map-get? user-reputation user-to-rate)) err-user-to-rate-inexistent)
    (let ((user-to-rate-reputation (get-user-reputation user-to-rate)))
      (asserts! (not (is-eq tx-sender user-to-rate)) err-same-user)
      (asserts! (or 
                  (< reputation-amount -100) (> reputation-amount 100) 
                  (< (+ user-to-rate-reputation reputation-amount) -100) (> (+ user-to-rate-reputation reputation-amount) 100)) 
                  err-invalid-reputation-amount)
      (map-set user-reputation user-to-rate {reputation: (+ user-to-rate-reputation reputation-amount)})
      (ok true)
    )
  )
)

(define-read-only (get-user-reputation (user principal)) 
  (default-to 0 (get reputation (map-get? user-reputation user)))
)