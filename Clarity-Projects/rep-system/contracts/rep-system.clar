(define-constant err-user-already-existent (err u101))
(define-constant err-same-user (err u102))
(define-constant err-invalid-reputation-amount (err u103))
(define-constant err-owner-only (err u105))

(define-constant contract-owner tx-sender)

(define-map user-reputation principal {reputation: int})
(define-map ratings {user: principal, user-rated: principal} {reputation: int, height: uint})

(define-map better-reputation principal {rated-users: (list 200 principal), rated-amounts:  (list 200 int), heights: (list 200 uint)})

(define-public (rate-user (user-to-rate principal) (reputation-amount int))
  (begin 
    (asserts! (is-eq contract-caller contract-owner) err-owner-only)
    (let (
      (user-to-rate-reputation (default-to 0 (get-user-reputation user-to-rate)))
      (existing-data (default-to 
        {rated-users: (list), rated-amounts: (list), heights: (list)} 
        (map-get? better-reputation tx-sender)
      ))
    )
      ;; (print (element-at (get rated-amounts existing-data) u2))
      (asserts! (not (is-eq tx-sender user-to-rate)) err-same-user)
      (asserts! (and 
                  (>= reputation-amount -10) (<= reputation-amount 10) 
                  (>= (+ user-to-rate-reputation reputation-amount) -100) (<= (+ user-to-rate-reputation reputation-amount) 100) 
                ) err-invalid-reputation-amount)
      (map-set user-reputation user-to-rate {reputation: (+ user-to-rate-reputation reputation-amount)})
      (map-set better-reputation tx-sender {
        rated-users: (unwrap-panic (as-max-len? (concat (get rated-users existing-data) (list user-to-rate)) u200)),
        rated-amounts: (unwrap-panic (as-max-len? (concat (get rated-amounts existing-data) (list reputation-amount)) u200)),
        heights: (unwrap-panic (as-max-len? (concat (get heights existing-data) (list stacks-block-height)) u200))
      })
      (ok true)
    )
  )
)

(define-read-only (get-user-reputation (user principal)) 
  (get reputation (map-get? user-reputation user))
)

(define-read-only (get-user-made-ratings (user principal))
  (default-to 
    {rated-users: (list), rated-amounts: (list), heights: (list)}
    (map-get? better-reputation user)
  )
)