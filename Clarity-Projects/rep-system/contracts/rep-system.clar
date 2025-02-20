(define-constant err-owner-only (err u100))
(define-constant err-same-user (err u101))
(define-constant err-invalid-reputation-amount (err u102))
(define-constant err-maximum-ratings-reached (err u103))
(define-constant err-already-made-at-this-block-height (err u104))

(define-constant contract-owner tx-sender)

(define-map user-reputation principal {reputation: int})
(define-map user-ratings principal {ratings: (list 200 {user: principal, amount: int, height: uint})})

(define-public (rate-user (user-to-rate principal) (reputation-amount int))
    
  ;; (match (get-rated-height user-to-rate) 
  ;;   height (asserts! (>= stacks-block-height (+ height u1000)) err-already-made-at-this-block-height)
  ;;   ()
  ;; )
  
  (let ((rate-height (get-rated-height user-to-rate)))
    (asserts! (>= stacks-block-height (if (is-none rate-height) stacks-block-height (+ (default-to u0 rate-height) u1000))) err-already-made-at-this-block-height)
    (asserts! (is-eq contract-caller contract-owner) err-owner-only)
    (let ((user-to-rate-reputation (default-to 0 (get-user-reputation user-to-rate)))
          (existing-data (default-to {ratings: (list)} (map-get? user-ratings tx-sender)))
          (new-rating {user: user-to-rate, amount: reputation-amount, height: stacks-block-height}))
      (asserts! (not (is-eq tx-sender user-to-rate)) err-same-user)
      (asserts! (and 
                  (>= reputation-amount -10) (<= reputation-amount 10) 
                  (>= (+ user-to-rate-reputation reputation-amount) -100) (<= (+ user-to-rate-reputation reputation-amount) 100) 
                ) err-invalid-reputation-amount)
      (if (is-some (get-rated-height user-to-rate))
        (let ((idx (unwrap! (find-index user-to-rate) err-maximum-ratings-reached))
              (current-ratings (get ratings existing-data)))
          (map-set user-ratings tx-sender {ratings: 
            (default-to current-ratings (replace-at? current-ratings idx new-rating))}))
        (map-set user-ratings tx-sender {ratings:
          (unwrap! (as-max-len? 
            (concat (get ratings existing-data) (list new-rating)) 
            u200) 
          err-maximum-ratings-reached)})
      )
      (map-set user-reputation user-to-rate {reputation: (+ user-to-rate-reputation reputation-amount)})
      (ok (map-get? user-reputation user-to-rate))
    )
  )
)

(define-read-only (get-user-reputation (user principal)) 
  (get reputation (map-get? user-reputation user))
)

(define-read-only (get-ratings-made (user principal)) 
  (default-to
    {ratings: (list)}
    (map-get? user-ratings user)
  )
)

(define-read-only (get-rated-height (user-to-rate principal))
  (let ((ratings (get ratings (default-to {ratings: (list)} (map-get? user-ratings tx-sender)))))
    (match (index-of (map get-user ratings) user-to-rate)
      index (some (unwrap! (get height (element-at? ratings index)) none))
      none
    )
  )
)

(define-private (get-user (rating {user: principal, amount: int, height: uint}))
  (get user rating)
)

(define-read-only (find-index (user-to-find principal))
  (let ((ratings (get ratings (default-to {ratings: (list)} (map-get? user-ratings tx-sender)))))
    (index-of (map get-user ratings) user-to-find)
  )
)