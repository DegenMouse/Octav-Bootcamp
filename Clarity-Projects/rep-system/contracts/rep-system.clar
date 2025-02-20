(define-constant err-owner-only (err u100))
(define-constant err-same-user (err u101))
(define-constant err-invalid-reputation-amount (err u102))
(define-constant err-maximum-ratings-reached (err u103))
(define-constant err-already-made-at-this-block-height (err u104))

(define-map user-reputation principal {reputation: int})
(define-map user-ratings principal {ratings: (list 200 {user: principal, amount: int, height: uint})})
(define-map received-ratings principal {ratings: (list 200 {user: principal, amount: int, height: uint})})


(define-public (rate-user (user-to-rate principal) (reputation-amount int))
  (begin
    (asserts! (not (is-eq tx-sender user-to-rate)) err-same-user)
    (let 
      ((user-to-rate-reputation (default-to 0 (get-user-reputation user-to-rate)))
       (existing-data (default-to {ratings: (list)} (map-get? user-ratings tx-sender)))
       (existing-received-data (default-to {ratings: (list)} (map-get? received-ratings user-to-rate)))
       (new-rating {user: tx-sender, amount: reputation-amount, height: stacks-block-height}))
      (asserts! (and 
                  (>= reputation-amount -10) (<= reputation-amount 10) 
                  (>= (+ user-to-rate-reputation reputation-amount) -100) 
                  (<= (+ user-to-rate-reputation reputation-amount) 100)) 
                err-invalid-reputation-amount)
      (match (get-rated-height user-to-rate)
        height 
          (begin
            (asserts! (>= stacks-block-height (+ height u1000)) err-already-made-at-this-block-height)
            (let ((idx (unwrap! (find-index user-to-rate) err-maximum-ratings-reached))
                  (current-ratings (get ratings existing-data))
                  (previous-rating (unwrap! (element-at? current-ratings idx) err-maximum-ratings-reached)))
              (map-set user-ratings tx-sender {ratings: 
                (default-to current-ratings (replace-at? current-ratings idx new-rating))})
              (map-set user-reputation user-to-rate 
                {reputation: (+ (- user-to-rate-reputation (get amount previous-rating)) reputation-amount)}))
            )
        (begin 
          (map-set user-ratings tx-sender {ratings:
            (unwrap! (as-max-len? 
              (concat (get ratings existing-data) (list new-rating)) u200) 
            err-maximum-ratings-reached)})
          (map-set user-reputation user-to-rate {reputation: (+ user-to-rate-reputation reputation-amount)})
        )
      )

      (map-set received-ratings user-to-rate {ratings: (unwrap! (as-max-len? 
          (concat (get ratings existing-received-data) (list new-rating)) u200) 
          err-maximum-ratings-reached)})
      (ok (map-get? user-reputation user-to-rate))
    )
  )
)

(define-read-only (get-user-reputation (user principal)) 
  (get reputation (map-get? user-reputation user))
)

(define-read-only (get-ratings-made (user principal)) 
  (default-to {ratings: (list)} (map-get? user-ratings user))
)

(define-read-only (get-ratings-received (user principal)) 
  (default-to {ratings: (list)} (map-get? received-ratings user))
)

(define-read-only (get-rated-height (user-to-rate principal))
  (let ((ratings (get ratings (default-to {ratings: (list)} (map-get? received-ratings user-to-rate)))))
    (match (index-of (map get-user ratings) tx-sender)
      index (some (unwrap! (get height (element-at? ratings index)) none))
      none
    )
  )
)

(define-private (get-user (rating {user: principal, amount: int, height: uint}))
  (get user rating)
)

(define-read-only (find-index (user-to-find principal))
  (let ((ratings (get ratings (default-to {ratings: (list)} (map-get? received-ratings user-to-find)))))
    (index-of (map get-user ratings) tx-sender)
  )
)