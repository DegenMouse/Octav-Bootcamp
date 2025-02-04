;; Errors types
(define-constant owner-only (err u100))
(define-constant past-deadline (err u101))
(define-constant insuficient-deposit (err u102))
(define-constant goal-not-reached (err u103))
(define-constant amount-surpasses-goal (err u104))
(define-constant deadline-not-reached (err u105))
(define-constant failed-to-return-info (err u106))
(define-constant campaign-not-found (err u107))
(define-constant unauthurised (err u103))

(define-data-var campaign-nonce uint u0)
(define-data-var copy-tx principal tx-sender)

;; TODO: states for campaign

(define-map campaign
    uint
    {
        owner: principal,
        goal: uint,
        deadline: uint,
        title: (string-ascii 100),
        contributions: uint
    }
)

(define-map refunds 
    {campaign-id: uint, contributor: principal} 
    {amount: uint}
)

(define-public (new-campaign 
    (goal uint) 
    (deadline uint) 
    (title (string-ascii 100)))
    (let (
        (campaign-id (var-get campaign-nonce))
    )
        
        (asserts! (> deadline stacks-block-height) past-deadline)
        (asserts! (> goal u0) insuficient-deposit)

        ;; Save the campaign to the map
        (map-set campaign campaign-id {
            owner: tx-sender,
            goal: goal,
            deadline: deadline,
            title: title,
            contributions: u0
        })

        (var-set campaign-nonce (+ campaign-id u1))
        (ok campaign-id)
    )
)

(define-public (cancel-campaign (campaign-id uint))
    (let (
        (existing-campaign (unwrap! (map-get? campaign campaign-id) campaign-not-found))
        (owner (get owner existing-campaign))
    )
        ;; Ensure only the owner can cancel the campaign
        (asserts! (is-eq owner tx-sender) owner-only)

        ;; Delete the campaign
        (map-delete campaign campaign-id)

        (ok true)
    )
)

(define-read-only (get-campaign (campaign-id uint))
    (map-get? campaign campaign-id)
)

(define-read-only (get-refund (campaign-id uint) (contributor principal))
    (map-get? refunds {campaign-id: campaign-id, contributor: contributor})
)

(define-public (contribute (campaign-id uint) (amount uint))
  (let (
        (campaign-data (unwrap! (map-get? campaign campaign-id) failed-to-return-info))
        (goal (get goal campaign-data))
        (deadline (get deadline campaign-data))
        (owner (get owner campaign-data))
        (current-contributions (get contributions campaign-data))
    )
    (asserts! (< stacks-block-height deadline) past-deadline)
    (asserts! (> amount u0) insuficient-deposit)
    (asserts! (<= amount (- goal current-contributions)) amount-surpasses-goal)
    (try! (stx-transfer? amount tx-sender (as-contract tx-sender)))

    ;; Update contributions
    (map-set campaign campaign-id 
      (merge campaign-data { contributions: (+ current-contributions amount) }))
    
    (let ((existing-refund (default-to  {amount: u0} (map-get? refunds {campaign-id: campaign-id, contributor: tx-sender}))))
        (map-set refunds {campaign-id: campaign-id, contributor: tx-sender} 
            (merge existing-refund {amount: (+ amount (get amount existing-refund))})))
    
    (ok {
        campaign: {
            id: campaign-id,
            owner: owner,
            goal: goal,
            deadline: deadline,
            contributions: (+ current-contributions amount)
        },
        refund: {
            amount: amount,
            contributor: tx-sender
        }
    })
  )
)

(define-public (claim (campaign-id uint))
    (let (
        (campaign-data (unwrap! (map-get? campaign campaign-id) failed-to-return-info))
        (owner (get owner campaign-data))
        (contributions (get contributions campaign-data))
        (goal (get goal campaign-data))
        (deadline (get deadline campaign-data))
    )
        (asserts! (is-eq owner tx-sender) owner-only)
        (asserts! (is-eq contributions goal) goal-not-reached)
        (asserts! (< stacks-block-height deadline) deadline-not-reached)

        (as-contract (try! (stx-transfer? contributions (as-contract tx-sender) owner)))
        (map-delete campaign campaign-id)
        (ok true)
    )
)

(define-public (refund (campaign-id uint))
    (let (
        (refund-data (unwrap! (map-get? refunds {campaign-id: campaign-id, contributor: tx-sender}) failed-to-return-info))
        (amount (get amount refund-data))
        (campaign-data (unwrap! (map-get? campaign campaign-id) failed-to-return-info))
        (deadline (get deadline campaign-data))
    )
        (asserts! (>  stacks-block-height deadline) deadline-not-reached)

        (var-set copy-tx tx-sender)
        (as-contract (try! (stx-transfer? amount tx-sender (var-get copy-tx))))
        
        (map-delete refunds {campaign-id: campaign-id, contributor: tx-sender})
        (ok {
            amount: amount,
            contributor: tx-sender
        })
    )
)

(define-public (get-balance)
    (ok (stx-get-balance tx-sender))
)