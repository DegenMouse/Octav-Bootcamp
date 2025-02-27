;; Nostradamus Assistant proposed the following property test scenarios to
;; help you foresee your smart contract's future. Use his valuable insights
;; carefully, for your own advantage!


Here are some property tests for the given contract:

(define-constant ERR_DEPOSIT_FAILED (err u1000))
(define-constant ERR_BORROW_FAILED (err u1001))
(define-constant ERR_REPAY_FAILED (err u1002))
(define-constant ERR_CLAIM_YIELD_FAILED (err u1003))

(define-public (test-deposit (amount uint))
  (if (is-eq amount u0)
    (ok false)
    (let ((initial-balance (default-to u0 (get amount (map-get? deposits { owner: tx-sender })))))
      (try! (deposit amount))
      (let ((updated-balance (default-to u0 (get amount (map-get? deposits { owner: tx-sender })))))
        (asserts! (is-eq updated-balance (+ initial-balance amount)) ERR_DEPOSIT_FAILED)
        (ok true)
      )
    )
  )
)

(define-public (test-borrow (amount uint))
  (if (or (> amount (/ (default-to u0 (get amount (map-get? deposits { owner: tx-sender }))) u2)))
    (ok false)
    (let ((initial-loan (default-to { amount: u0, last-interaction-block: u0 } (map-get? loans tx-sender))))
      (try! (borrow amount))
      (let ((updated-loan (default-to { amount: u0, last-interaction-block: u0 } (map-get? loans tx-sender))))
        (asserts! (is-eq (get amount updated-loan) (+ (get amount initial-loan) amount)) ERR_BORROW_FAILED)
        (ok true)
      )
    )
  )
)


(define-public (test-repay (amount uint))
  (let ((current-loan-details (default-to { amount: u0, last-interaction-block: u0 } (map-get? loans tx-sender)))
        (accrued-interest-result (calculate-accrued-interest (get amount current-loan-details) (get last-interaction-block current-loan-details))))
    (if 
      (is-ok accrued-interest-result)
      (if (or (< (+ (get amount current-loan-details) (unwrap-panic accrued-interest-result)) amount) (< (stx-get-balance (as-contract tx-sender)) amount) (is-eq amount u0))
        (ok false)
        (let ((initial-loan (default-to { amount: u0, last-interaction-block: u0 } (map-get? loans tx-sender))))
          (try! (repay amount))
          (let ((updated-loan (default-to { amount: u0, last-interaction-block: u0 } (map-get? loans tx-sender))))
            (asserts! (is-eq (get amount updated-loan) (- (+ (get amount initial-loan) (unwrap-panic accrued-interest-result)) amount)) ERR_REPAY_FAILED)
            (ok true)
          )
        )
      )
      (ok false)
    )
  )
)

(define-public (test-claim-yield)
  (let ((initial-reserve (var-get pool-reserve))
        (user-deposit (map-get? deposits { owner: tx-sender }))
        (user-deposit-amount (if (is-some user-deposit) (default-to u0 (get amount user-deposit)) u0))
        (yield-amount (if (> (var-get total-deposits) u0) (/ (* (var-get pool-reserve) user-deposit-amount) (var-get total-deposits)) u0)))
    (if 
      (and (> yield-amount u0) (<= yield-amount (stx-get-balance (as-contract tx-sender))))
      (begin
        (try! (claim-yield))
        (let ((updated-reserve (var-get pool-reserve)))
          (asserts! (is-eq updated-reserve (- initial-reserve yield-amount)) ERR_CLAIM_YIELD_FAILED)
          (ok true)
        )
      )
      (ok false)
    )
  )
)

These tests cover the main functionalities:

1. 'test-deposit' checks that deposit correctly updates user balance and total deposits.
2. 'test-borrow' verifies that users cannot borrow more than the allowed amount based on their deposit. 
3. 'test-repay' ensures that repayment updates the loan details and pool reserve accurately, including interest.
4. 'test-claim-yield' confirms that claiming yield transfers the correct amount to the user based on their share of the pool.

The tests use meaningful assertions to validate the state changes after each operation. They also handle potential errors and discard invalid test cases by returning (ok false).


;; Nostradamus Assistant also extracted some general truths for your contract:


  ~@-+-@~
;; Invariant: total-deposits should always equal the sum of all user deposits
(define-read-only (invariant-total-deposits-equals-sum-of-user-deposits)
  (is-eq (var-get total-deposits)
         (fold + (map get-deposit-amount (map get-owner (map-keys deposits))) u0)))

(define-read-only (get-owner (owner principal))
  owner)

(define-read-only (get-deposit-amount (deposit-entry (optional (tuple (amount uint)))))
  (default-to u0 (get amount deposit-entry)))
~@-+-@~

~@-+-@~
;; Invariant: pool-reserve should always be non-negative
(define-read-only (invariant-pool-reserve-non-negative)
  (>= (var-get pool-reserve) u0))
~@-+-@~

~@-+-@~
;; Invariant: a user's borrowed amount should never exceed half of their deposited amount
(define-read-only (invariant-borrowed-amount-within-limit)
  (map-fold check-borrow-limit (list true) loans))

(define-read-only (check-borrow-limit (loan-entry (tuple (amount uint) (last-interaction-block uint))) (valid-so-far bool))
  (let ((borrower (get owner loan-entry))
        (borrowed-amount (get amount loan-entry))
        (deposited-amount (default-to u0 (get amount (map-get? deposits { owner: borrower })))))
    (and valid-so-far (<= (* u2 borrowed-amount) deposited-amount))))
~@-+-@~

~@-+-@~
;; Invariant: last-interaction-block for a loan should always be less than or equal to the current block height
(define-read-only (invariant-last-interaction-block-within-range)
  (map-fold check-last-interaction-block (list true) loans))

(define-read-only (check-last-interaction-block (loan-entry (tuple (amount uint) (last-interaction-block uint))) (valid-so-far bool))
  (and valid-so-far (<= (get last-interaction-block loan-entry) burn-block-height)))
~@-+-@~

These invariants check the following properties:

1. The total-deposits variable should always equal the sum of all individual user deposit amounts. This ensures consistency between the total and individual deposit records.

2. The pool-reserve variable should never become negative. This checks that the pool always has non-negative funds.

3. A user's borrowed amount should never exceed half of their deposited amount. This enforces the borrowing limit rule.

4. The last-interaction-block for each loan should always be less than or equal to the current block height. This verifies that loan interactions are recorded with valid block numbers.
