(define-constant err-invalid (err u300))
(define-constant err-list-length-exceeded (err u101))
(define-constant err-already-asked-to-join (err u102))
(define-constant err-already-joined (err u103))
(define-constant err-not-in-miner-map (err u104))
(define-constant err-no-vote-permission (err u105))
(define-constant err-more-blocks-to-pass (err u106))
(define-constant err-no-pending-miners (err u107))
(define-constant err-already-voted (err u108))
(define-constant err-not-asked-to-join (err u109))
(define-constant err-cant-unwrap-check-miner (err u110))
(define-constant err-cant-unwrap-asked-to-join (err u111))
(define-constant err-cant-unwrap-block-info (err u112))
(define-constant err-currently-notifier (err u113))
(define-constant err-not-in-miner-map-miner-to-remove (err u114))
(define-constant err-already-proposed-for-removal (err u116))
(define-constant err-not-proposed-for-removal (err u117))
(define-constant err-cant-remove-when-alone-in-pool (err u118))
(define-constant err-cant-vote-himself (err u119))
(define-constant err-cant-change-notifier (err u120))
(define-constant err-already-proposed-for-notifier (err u121))
(define-constant err-not-proposed-for-removal-proposal-block-missing (err u122))
(define-constant err-not-proposed-for-notifier-k-missing (err u123))
(define-constant err-not-proposed-notifier (err u124))
(define-constant err-already-notifier (err u125))
(define-constant err-not-in-miner-map-proposed-notifier (err u126))
(define-constant err-vote-started-already (err u127))
(define-constant err-voting-still-active (err u128))
(define-constant err-not-voting-period (err u129))
(define-constant err-not-waiting (err u130)) 
(define-constant err-not-pending (err u131))
(define-constant err-no-join-block-data (err u132))
(define-constant err-not-voted (err u133))
(define-constant err-only-notifier (err u134))
(define-constant err-one-warning-per-block (err u135))
(define-constant err-stacks-block-height-invalid (err u136))
(define-constant err-unwrap-miner-index (err u999))
(define-constant err-insufficient-balance (err u1001))
(define-constant err-missing-balance (err u1002))
(define-constant err-already-distributed (err u1003))
(define-constant err-cant-unwrap-rewarded-block (err u1004))

(define-constant notifier-election-blocks-to-pass u144)
(define-constant blocks-to-pass u100)
(define-constant k-percentage u67)
(define-constant k-critical u75)

(define-map balance principal uint)
(define-map claimed-rewards { block-number: uint } { claimed: bool })
(define-map map-is-miner { address: principal } { value: bool })
(define-map map-is-waiting { address: principal } { value: bool })
(define-map map-is-pending { address: principal } { value: bool })
(define-map map-is-proposed-for-removal { address: principal } { value: bool })
(define-map map-block-asked-to-join { address: principal } { value: uint })
(define-map map-block-proposed-to-remove { address: principal } { value: uint })
(define-map map-block-joined { address: principal } { stacks-block-height: uint })
(define-map map-balance-xBTC { address: principal } { value: uint })
(define-map auto-exchange { address: principal } { value: bool })
(define-map btc-address { address: principal } { btc-address: {hashbytes: (buff 32), version: (buff 1)} })

(define-map map-votes-accept-join { address: principal } { value: uint })
(define-map map-votes-reject-join { address: principal } { value: uint })
(define-map map-votes-accept-removal { address: principal } { value: uint })
(define-map map-votes-reject-removal { address: principal } { value: uint })
(define-map map-join-request-voter { miner-to-vote: principal, voter: principal } { value: bool })
(define-map map-remove-request-voter { miner-to-vote: principal, voter: principal } { value: bool })
(define-map map-voted-update-notifier { miner-who-voted: principal } { miner-voted: principal })
(define-map map-votes-notifier { voted-notifier: principal } { votes-number: uint })
(define-map map-blacklist { address: principal } { value: bool })
(define-map map-total-withdraw { address: principal } { value: uint })
(define-map map-warnings { address: principal } { value: uint })

(define-data-var miners-list-len-at-reward-block uint u0)
(define-data-var notifier principal tx-sender)
(define-data-var waiting-list (list 300 principal) (list ))
(define-data-var miners-list (list 300 principal) (list (var-get notifier)))
(define-data-var miners-list-bitcoin (list 300 {hashbytes: (buff 32), version: (buff 1)}) (list ))
(define-data-var pending-accept-list (list 300 principal) (list ))
(define-data-var proposed-removal-list (list 300 principal) (list ))
(define-data-var n uint u1)
(define-data-var k uint u1)
(define-data-var waiting-list-miner-to-remove principal tx-sender) ;; use in remove-principal-miners-list
(define-data-var pending-accept-list-miner-to-remove principal tx-sender)
(define-data-var miners-list-miner-to-remove principal tx-sender)
(define-data-var proposed-removal-list-miner-to-remove principal tx-sender)
(define-data-var last-join-done uint u1)
(define-data-var miner-to-remove-votes-join principal tx-sender)
(define-data-var miner-to-remove-votes-remove principal tx-sender)
(define-data-var notifier-previous-entries-removed bool true)
(define-data-var notifier-vote-active bool false)
(define-data-var notifier-vote-start-block uint u0)
(define-data-var notifier-vote-end-block uint u0)
(define-data-var max-votes-notifier uint u0)
(define-data-var max-voted-proposed-notifier principal tx-sender)
(define-data-var reward uint u0)
(define-data-var total-rewarded uint u0)
(define-data-var blocks-won uint u0)
(define-data-var reward-change-funds uint u0)
(define-data-var temp-distributed-change-funds uint u0)
(define-data-var temp-change-after-flushing uint u0)
(define-data-var temp-miners-bitcoin-lists (list 300 {hashbytes: (buff 32), version: (buff 1)}) (list ))
;; equivalent to 70 USD in sats * 1_000_000
(define-data-var pool-spend-amount-sats-per-block uint u3730000000)

(map-set map-is-miner {address: tx-sender} {value: true})
(map-set map-block-joined {address: tx-sender} {stacks-block-height: stacks-block-height})
(map-set balance tx-sender u0)
;; at new join -> block height - last-join-done >= 100 !

;; READ ONLY FE UTILS

;; waiting miners

(define-private (get-k-at-block-proposed-removal (miner-to-vote principal)) 
(let ((block-proposed-to-remove (get value (map-get? map-block-proposed-to-remove {address: miner-to-vote}))))
  (asserts! (is-some block-proposed-to-remove) err-not-proposed-for-removal)
  (if 
    (is-eq (unwrap-panic 
      block-proposed-to-remove) 
      stacks-block-height) 
    (ok (var-get k)) 
    (at-block 
      (unwrap-panic 
        (get-stacks-block-info? id-header-hash 
          (unwrap-panic block-proposed-to-remove))) 
            (ok (var-get k))))))

(define-private (get-n-at-block-proposed-removal (miner-to-vote principal))
(let ((block-proposed-to-remove (get value (map-get? map-block-proposed-to-remove {address: miner-to-vote})))) 
  (asserts! (is-some block-proposed-to-remove) err-not-proposed-for-removal)
  (if 
    (is-eq (unwrap-panic 
      block-proposed-to-remove) 
      stacks-block-height) 
    (ok (var-get n)) 
    (at-block 
      (unwrap-panic 
        (get-stacks-block-info? id-header-hash 
          (unwrap-panic block-proposed-to-remove))) 
            (ok (var-get n))))))


(define-read-only (get-all-data-waiting-miners (waiting-miners-list (list 100 principal))) 
(map get-all-data-waiting-miner waiting-miners-list))

(define-private (get-all-data-waiting-miner (miner principal))
(let ((k-at-block-asked-to-join (unwrap-panic (get-k-at-block-asked-to-join miner)))
      (n-at-block-asked-to-join (unwrap-panic (get-n-at-block-asked-to-join miner))))
  (begin 
    (asserts! (is-some (get value (map-get? map-is-waiting {address: miner}))) err-not-waiting)
    (ok 
      {
        pos-votes: 
          (default-to u0 (get value (map-get? map-votes-accept-join {address: miner}))),
        pos-thr: 
          (if 
            (is-eq k-at-block-asked-to-join u0) 
            u1 
            k-at-block-asked-to-join),
        neg-votes:
          (default-to u0 (get value (map-get? map-votes-reject-join {address: miner}))),
        neg-thr: 
          (if   
            (is-eq n-at-block-asked-to-join u1) 
            u1 
            (if 
              (is-eq n-at-block-asked-to-join u2) 
              u2 
              (+ (- n-at-block-asked-to-join k-at-block-asked-to-join) u1)))}))))


(define-private (check-is-miner-now (miner principal))
(default-to false (get value (map-get? map-is-miner {address: miner}))))

;; miners proposed for removal

(define-private (check-is-miner-when-requested-join (miner-to-vote principal))
(let (
  (asked-to-join-height (unwrap! (get value (map-get? map-block-asked-to-join {address: miner-to-vote})) err-cant-unwrap-asked-to-join))
  (is-asked-to-join-height (is-eq asked-to-join-height stacks-block-height))
  (asked-to-join-header-hash (unwrap! (get-stacks-block-info? id-header-hash asked-to-join-height) err-cant-unwrap-block-info)))
  (ok
    (default-to false
      (if is-asked-to-join-height
      (get value (map-get? map-is-miner {address: contract-caller}))
        (at-block asked-to-join-header-hash (get value (map-get? map-is-miner {address: contract-caller}))))))))

(define-private (check-is-miner-when-requested-remove (miner-to-vote principal))
(ok 
  (if 
    (is-some 
      (if 
        (is-eq  
          (unwrap! (get value (map-get? map-block-proposed-to-remove {address: miner-to-vote})) err-cant-unwrap-asked-to-join) 
          stacks-block-height)
        (get value (map-get? map-is-miner {address: contract-caller})) 
        (at-block 
          (unwrap! 
            (get-stacks-block-info? id-header-hash 
              (unwrap! (get value (map-get? map-block-proposed-to-remove {address: miner-to-vote})) err-cant-unwrap-asked-to-join)) 
          err-cant-unwrap-block-info) 
          (get value (map-get? map-is-miner {address: contract-caller})))))
  (if 
      (is-eq 
        (unwrap! 
          (get value (map-get? map-block-proposed-to-remove {address: miner-to-vote})) 
        err-cant-unwrap-asked-to-join) 
        stacks-block-height) 
      (unwrap-panic (get value (map-get? map-is-miner {address: contract-caller}))) 
      (at-block
        (unwrap! 
          (get-stacks-block-info? id-header-hash 
            (unwrap-panic (get value (map-get? map-block-proposed-to-remove {address: miner-to-vote}))))
          err-cant-unwrap-block-info)
        (unwrap-panic (get value (map-get? map-is-miner {address: contract-caller})))))
  false)))


(define-private (check-is-pending-now (miner principal))
(default-to false (get value (map-get? map-is-pending {address: miner}))))

(define-read-only (get-address-status (address principal))
(if (check-is-miner-now address)
  (ok "is-miner")
  (if (check-is-waiting-now address)
    (ok "is-waiting")
    (if (check-is-pending-now address)
      (ok "is-pending")
      (ok "is-none")))))


(define-read-only (get-all-data-miners-proposed-for-removal (removal-miners-list (list 100 principal))) 
(map get-all-data-miner-proposed-for-removal removal-miners-list))

(define-private (get-all-data-miner-proposed-for-removal (miner principal)) 
(let ((k-at-block-proposed-removal (unwrap-panic (get-k-at-block-proposed-removal miner)))
      (n-at-block-proposed-removal (unwrap-panic (get-n-at-block-proposed-removal miner))))
  (begin 
    (asserts! (is-some (get value (map-get? map-is-proposed-for-removal {address: miner}))) err-not-pending)
    (ok 
      {
        vts-for: 
          (default-to u0 (get value (map-get? map-votes-accept-removal {address: miner}))),
        pos-thr: 
          (if 
            (is-eq k-at-block-proposed-removal u0) 
            u1 
            k-at-block-proposed-removal),
        vts-against: 
          (default-to u0 (get value (map-get? map-votes-reject-removal {address: miner}))),
        neg-thr: 
          (if   
            (is-eq n-at-block-proposed-removal u2) 
            u1 
            (if (is-eq n-at-block-proposed-removal u3) 
              u2 
              (+ (- n-at-block-proposed-removal k-at-block-proposed-removal) u1)))}))))

;; pending accept miners

(define-read-only (get-all-data-miners-pending-accept (pending-miners-list (list 100 principal))) 
(map get-data-miner-pending-accept pending-miners-list))

(define-private (get-data-miner-pending-accept (miner principal)) 
(begin 
  (asserts! (is-some (get value (map-get? map-is-pending {address: miner}))) err-not-pending)
  (ok 
    {
      miner: miner,
      remaining-blocks-until-join: (get-remaining-blocks-until-join)
    })))

(define-read-only (get-remaining-blocks-until-join)
  (if (> blocks-to-pass (- stacks-block-height (var-get last-join-done)))
    (- blocks-to-pass (- stacks-block-height (var-get last-join-done)))
    u0
  )
)

;; blocks number as miner
(define-read-only (get-all-data-miners-blocks (local-miners-list (list 100 principal))) 
(map get-data-miner-blocks local-miners-list))
(define-private (get-data-miner-blocks (miner principal)) 
(begin 
  (asserts! (is-some (get value (map-get? map-is-miner {address: miner}))) err-not-in-miner-map)
  (asserts! (is-some (get stacks-block-height (map-get? map-block-joined {address: miner}))) err-no-join-block-data)
  (ok 
    {
      miner: miner,
      blocks-as-miner: (- stacks-block-height (unwrap-panic (get stacks-block-height (map-get? map-block-joined {address: miner}))))
    })))

;; miners in pool

(define-read-only (get-all-data-miners-in-pool (local-miners-list (list 100 principal))) 
(map get-data-miner-in-pool local-miners-list))

(define-private (get-data-miner-in-pool (miner principal)) 
(begin 
  (asserts! (is-some (get value (map-get? map-is-miner {address: miner}))) err-not-in-miner-map)
  (asserts! (is-some (get stacks-block-height (map-get? map-block-joined {address: miner}))) err-no-join-block-data)
  (ok 
    {
      blocks-as-miner: (- stacks-block-height (unwrap-panic (get stacks-block-height (map-get? map-block-joined {address: miner})))),
      was-blacklist: (default-to false (get value (map-get? map-blacklist {address: miner}))),
      warnings: (default-to u0 (get value (map-get? map-warnings {address: miner}))),
    })))

;; total withdrawals

(define-read-only (get-all-data-total-withdrawals (local-miners-list (list 100 principal))) 
(map get-data-miner-withdrawals local-miners-list))

(define-private (get-data-miner-withdrawals (miner principal)) 
(ok 
  (default-to u0 (get value (map-get? map-total-withdraw {address: miner})))
  ))

;; notifier

(define-read-only (get-data-notifier-election-process)
{
  vote-status: (var-get notifier-vote-active), 
  election-blocks-remaining:
    (if (<= (var-get notifier-vote-end-block) stacks-block-height)
    u0
    (- (var-get notifier-vote-end-block) stacks-block-height))})

(define-read-only (get-all-data-notifier-voter-miners (voter-miners-list (list 100 principal)))
(map get-data-notifier-voter-miner voter-miners-list))

(define-private (get-data-notifier-voter-miner (miner principal)) 
(begin 
  (asserts! (is-some (get miner-voted (map-get? map-voted-update-notifier {miner-who-voted: miner}))) err-not-voted)
  (ok 
    {
      miner: miner,
      voted-notifier: 
      (unwrap-panic (get miner-voted (map-get? map-voted-update-notifier {miner-who-voted: miner}))) 
    })))

;; balances 

(define-read-only (was-block-claimed (given-stacks-block-height uint)) 
  (if 
    (is-none (get claimed (map-get? claimed-rewards {block-number: given-stacks-block-height}))) 
    false 
    (if 
      (unwrap-panic (get claimed (map-get? claimed-rewards {block-number: given-stacks-block-height}))) 
      true 
      false)))

(define-read-only (get-reward-at-block-read (block-number uint)) 
{
  reward: (get-stacks-block-info? coinbase-total-ustx block-number), 
  claimer: (get-stacks-block-info? miner-address block-number)
})

;; BALANCES FLOW

(define-read-only (check-is-miner-when-requested-join-tool-fn (miner-to-vote-address principal)) 
(get value (map-get? map-is-miner {address: contract-caller})))

(define-private (check-is-proposed-for-removal-now (miner principal))
(default-to false (get value (map-get? map-is-proposed-for-removal {address: miner}))))

(define-private (check-is-waiting-now (miner principal))
(default-to false (get value (map-get? map-is-waiting {address: miner}))))

;; read balance
(define-read-only (get-balance (address principal)) 
(map-get? balance address))

(define-read-only (get-miner-btc-address (miner-address principal))
  (map-get? btc-address {address: miner-address}))

(define-public (set-my-btc-address (new-btc-address {hashbytes: (buff 32), version: (buff 1)})) 
  (ok (map-set btc-address {address: contract-caller} {btc-address: new-btc-address})))

(define-private (remove-principal-waiting-list (miner principal))
(begin
  (var-set waiting-list-miner-to-remove miner) 
  (ok (filter is-principal-in-waiting-list (var-get waiting-list))))) 

(define-private (remove-principal-pending-accept-list (miner principal))
(begin 
  (var-set waiting-list-miner-to-remove miner) 
  (ok (filter is-principal-in-pending-accept-list (var-get pending-accept-list)))))

(define-private (remove-principal-miners-list (miner principal))
(begin
  (var-set miners-list-miner-to-remove miner) 
  (ok (filter is-principal-in-miners-list (var-get miners-list)))))

(define-private (remove-principal-proposed-removal-list (miner principal))
(begin
  (var-set proposed-removal-list-miner-to-remove miner) 
  (ok (filter is-principal-in-proposed-removal-list (var-get proposed-removal-list)))))

;; MINER STATUS FUNCTIONS

;; READ-ONLY UTILS

(define-read-only (get-k) 
(var-get k))

(define-read-only (get-n) 
(var-get n))

(define-read-only (get-notifier) 
(var-get notifier))

(define-read-only (get-blocks-won) 
(var-get blocks-won))

(define-read-only (get-total-rewards-distributed) 
(var-get total-rewarded))

(define-read-only (get-waiting-list) 
(var-get waiting-list))

(define-read-only (get-waiting-list-positions (index1 uint) (index2 uint)) 
(default-to (list ) (slice? (var-get waiting-list) index1 index2)))

(define-read-only (get-pending-accept-list) 
(var-get pending-accept-list))

(define-read-only (get-pending-accept-list-positions (index1 uint) (index2 uint)) 
(default-to (list ) (slice? (var-get pending-accept-list) index1 index2)))

(define-read-only (get-miners-list) 
(var-get miners-list))

(define-read-only (get-miners-list-positions (index1 uint) (index2 uint)) 
(default-to (list ) (slice? (var-get miners-list) index1 index2)))

(define-read-only (get-proposed-removal-list) 
(var-get proposed-removal-list))

(define-read-only (get-notifier-vote-status) 
(var-get notifier-vote-active))

(define-read-only (get-notifier-vote-number (voted-notifier principal)) 
(get votes-number (map-get? map-votes-notifier {voted-notifier: voted-notifier})))

(define-read-only (get-warnings-user (user principal)) 
(default-to u0 (get value (map-get? map-warnings {address: user}))))

(define-read-only (is-blacklisted (miner principal)) 
(default-to false (get value (map-get? map-blacklist {address: miner}))))

(define-read-only (is-claimed (block uint)) 
(default-to false (get claimed (map-get? claimed-rewards {block-number: block}))))

(define-read-only (get-max-voted-notifier) 
(var-get max-voted-proposed-notifier))

(define-read-only (get-max-votes-notifier) 
(var-get max-votes-notifier))

(define-read-only (get-current-block)
(ok stacks-block-height))

(define-read-only (blocks-passed-for-pending-miners)
(if (>= (- stacks-block-height (var-get last-join-done)) blocks-to-pass)
    true
    false))

(define-private (is-principal-in-waiting-list (miner principal))
(not (is-eq 
  (var-get waiting-list-miner-to-remove)
  miner)))

(define-private (is-principal-in-pending-accept-list (miner principal))
(not (is-eq 
  (var-get pending-accept-list-miner-to-remove)
  miner)))

(define-private (is-principal-in-miners-list (miner principal))
(not (is-eq  
  (var-get miners-list-miner-to-remove) 
  miner)))

(define-private (is-principal-in-proposed-removal-list (miner principal))
(not (is-eq  
  (var-get proposed-removal-list-miner-to-remove) 
  miner)))

;; returns the sats amount the whole pool has to send to the FROST in order to confirm the block (multiplied by ONE_6)
(define-read-only (get-pool-total-spend-per-block)
(var-get pool-spend-amount-sats-per-block))

;; deposit funds
(define-public (deposit-stx (amount uint))
(let ((sender contract-caller)
      (balance-sender (map-get? balance sender)))
  (try! (stx-transfer? amount contract-caller (as-contract tx-sender)))
  (if (is-none balance-sender) 
    (ok (map-set balance sender amount))
    (ok (map-set balance sender (+ (unwrap! balance-sender err-missing-balance) amount))))))

;; withdraw funds
(define-public (withdraw-stx (amount uint)) 
(let ((receiver contract-caller)) 
  (asserts! (>= (unwrap! (map-get? balance receiver) err-missing-balance) amount) err-insufficient-balance)
  (try! (as-contract (stx-transfer? amount tx-sender receiver)))
  (if 
    (is-some (get value (map-get? map-total-withdraw {address: receiver}))) 
    (map-set map-total-withdraw {address: receiver} {value: (+ (unwrap-panic (get value (map-get? map-total-withdraw {address: receiver}))) amount)}) 
    (map-set map-total-withdraw {address: receiver} {value: amount}))
  (ok (map-set balance receiver (- (unwrap! (map-get? balance receiver) err-missing-balance) amount)))))

;; exchange funds
(define-public (set-auto-exchange (new-value bool)) 
  (ok (map-set auto-exchange {address: contract-caller} {value: new-value})))

(define-read-only (get-auto-exchange (address principal)) 
  (map-get? auto-exchange {address: address}))

(define-public (reward-distribution (block-number uint))
(begin 
  (asserts! (< block-number stacks-block-height) err-stacks-block-height-invalid) ;; +100  ? 
  (asserts! (is-none (get claimed (map-get? claimed-rewards {block-number: block-number}))) err-already-distributed)
  (let ((miners-list-at-reward-block 
          (at-block (unwrap! (get-stacks-block-info? id-header-hash block-number) err-cant-unwrap-rewarded-block) (var-get miners-list)))
        (block-reward (get-reward-at-block-read block-number))
        (current-reward (var-get reward))
        (current-miners-list-len (len miners-list-at-reward-block))
        (distribution-change-funds (mod current-reward current-miners-list-len))
        (current-change-funds (var-get reward-change-funds)))
    (map-set claimed-rewards {block-number: block-number} {claimed: true})
    (var-set miners-list-len-at-reward-block current-miners-list-len) 
    (var-set reward (unwrap-panic (get reward block-reward)))
    (var-set total-rewarded (+ (var-get total-rewarded) (var-get reward)))
    (var-set blocks-won (+ (var-get blocks-won) u1))
    (var-set reward-change-funds (+ current-change-funds distribution-change-funds))
    (map distribute-reward-each-miner miners-list-at-reward-block)
    (ok true))))

(define-private (distribute-reward-each-miner (miner principal)) 
(let ((miner-balance (default-to u0 (map-get? balance miner)))
      (current-reward (var-get reward))
      (current-change-funds (var-get reward-change-funds))
      (current-miners-list-len (var-get miners-list-len-at-reward-block))
      (distributed-amount (/ current-reward current-miners-list-len))
      ) 
  
  (map-set balance miner 
    (+ miner-balance distributed-amount))))

(define-private (flush-change-each-miner (miner principal)) 
(let ((miner-balance (default-to u0 (map-get? balance miner)))
      (current-reward (var-get reward))
      (current-change-funds (var-get reward-change-funds))
      (current-miners-list-len (var-get miners-list-len-at-reward-block))
      (distributed-amount (/ current-reward current-miners-list-len))
      (distribution-change-funds (mod current-reward current-miners-list-len))) 
  (var-set reward-change-funds (+ current-change-funds distribution-change-funds))
  (map-set balance miner 
    (+ miner-balance distributed-amount))))

;; JOINING FLOW

(define-public (ask-to-join (my-btc-address {hashbytes: (buff 32), version: (buff 1)}))
(begin 
  (asserts! (not (check-is-miner-now contract-caller)) err-already-joined) 
  (asserts! (not (check-is-waiting-now contract-caller)) err-already-asked-to-join) 
  (map-set map-block-asked-to-join {address: contract-caller} {value: stacks-block-height})
  (map-set btc-address {address: contract-caller} {btc-address: my-btc-address})
  (var-set waiting-list (unwrap-panic (as-max-len? (append (var-get waiting-list) tx-sender) u300)))
  (map-set map-is-waiting {address: contract-caller} {value: true})
  (ok true)))

(define-private (is-vote-accepted (votes-number uint) (k-local uint))
(if 
  (is-eq k-local u0) ;; k is 0 for n=1, n=2
    (>= votes-number u1) 
    (>= votes-number k-local)))

(define-private (is-vote-rejected-join (votes-number uint) (k-local uint) (n-local uint))
(if 
  (is-eq n-local u1) 
  (>= votes-number u1) 
  (if (is-eq n-local u2) 
    (>= votes-number u2) 
    (>= votes-number (+ (- n-local k-local) u1)))))

(define-public (vote-positive-join-request (miner-to-vote principal))
(begin
  (asserts! (check-is-waiting-now miner-to-vote) err-not-asked-to-join) ;; map_is_waiting
    (asserts! (unwrap! (check-is-miner-when-requested-join miner-to-vote) err-cant-unwrap-check-miner) err-no-vote-permission)
    (asserts! (not (not-voted-join miner-to-vote)) err-already-voted) ;; O(1)
    (map-set map-join-request-voter 
      {miner-to-vote: miner-to-vote, voter: contract-caller} 
      {value: true})
    (if (is-some (get value (map-get? map-votes-accept-join {address: miner-to-vote}))) 
      (map-set map-votes-accept-join {address: miner-to-vote} {value: (+ (unwrap-panic (get value (map-get? map-votes-accept-join {address: miner-to-vote}))) u1)})
      (map-set map-votes-accept-join {address: miner-to-vote} {value: u1}))
    (ok true)))

(define-public (vote-negative-join-request (miner-to-vote principal))
(begin
  (asserts! (check-is-waiting-now miner-to-vote) err-not-asked-to-join)
    (asserts! (unwrap! (check-is-miner-when-requested-join miner-to-vote) err-cant-unwrap-check-miner) err-no-vote-permission)
    (asserts! (not-voted-join miner-to-vote) err-already-voted)    
    (map-set map-join-request-voter 
      {miner-to-vote: miner-to-vote, voter: contract-caller} 
      {value: true})
    (if (is-some (get value (map-get? map-votes-reject-join {address: miner-to-vote}))) 
      (map-set map-votes-reject-join {address: miner-to-vote} {value: (+ (unwrap-panic (get value (map-get? map-votes-reject-join {address: miner-to-vote}))) u1)})
      (map-set map-votes-reject-join {address: miner-to-vote} {value: u1}))  
  (if (is-vote-rejected-join (unwrap-panic (get value (map-get? map-votes-reject-join {address: miner-to-vote}))) (unwrap-panic (get-k-at-block-asked-to-join miner-to-vote)) (unwrap-panic (get-n-at-block-asked-to-join miner-to-vote)))
    (begin
      (try! (reject-miner-in-pool miner-to-vote))
      (ok true))
    (ok true)))

(define-public (quit-waiting-list) 
  (begin 
    (asserts! (check-is-waiting-now contract-caller) err-not-asked-to-join)
    (asserts! 
      (<= 
        blocks-to-pass
        (- stacks-block-height 
          (default-to stacks-block-height 
            (get value 
              (map-get? map-block-asked-to-join {address: contract-caller}))))) err-more-blocks-to-pass)
    (let ((remove-result (unwrap-panic (remove-principal-waiting-list contract-caller))))
      (var-set miner-to-remove-votes-join contract-caller)
      (var-set waiting-list remove-result)
      (map-delete map-is-waiting {address: contract-caller})
      (ok (clear-votes-map-join-vote contract-caller)))))

(define-private (accept-miner-in-pool (miner principal)) 
(let ((pending-accept-result (as-max-len? (append (var-get pending-accept-list) miner) u300)))
  (asserts! (is-some pending-accept-result) err-list-length-exceeded) ;; O(1) 
  (map-set map-warnings {address: miner} {value: u0})
  (map-set balance miner u0)
  (var-set miner-to-remove-votes-join miner)
  (var-set waiting-list (unwrap-panic (as-max-len? (unwrap-panic (remove-principal-waiting-list miner)) u300))) ;; O(N)
  (map-delete map-is-waiting {address: miner})
  (map-set map-is-pending {address: miner} {value: true})
  (clear-votes-map-join-vote miner)
  (ok (var-set pending-accept-list (unwrap-panic pending-accept-result)))))

(define-private (reject-miner-in-pool (miner principal)) 
(let ((remove-result (unwrap-panic (remove-principal-waiting-list miner))))
  (var-set miner-to-remove-votes-join miner)
  (var-set waiting-list remove-result)
  (map-delete map-is-waiting {address: miner})
  (clear-votes-map-join-vote miner)
  true))

(define-private (clear-votes-map-join-vote (miner principal)) 
(begin 
  (map-delete map-votes-accept-join {address: (var-get miner-to-remove-votes-join)})
  (map-delete map-votes-reject-join {address: (var-get miner-to-remove-votes-join)})
  (map-delete map-block-asked-to-join {address: (var-get miner-to-remove-votes-join)})
  (map remove-map-record-join-vote (var-get miners-list))))

(define-private (remove-map-record-join-vote (miner principal))
(map-delete map-join-request-voter {miner-to-vote: (var-get miner-to-remove-votes-join), voter: miner}))

(define-private (not-voted-join (miner principal)) 
(not (default-to false 
     (get value (map-get? map-join-request-voter 
       {miner-to-vote: miner, voter: contract-caller})))))

(define-public (try-enter-pool)
(begin 
  (asserts! (is-some (get value (map-get? map-votes-accept-join {address: contract-caller}))) err-not-asked-to-join)
  (if (is-vote-accepted (unwrap-panic (get value (map-get? map-votes-accept-join {address: contract-caller}))) (unwrap-panic (get-k-at-block-asked-to-join contract-caller)))
    (accept-miner-in-pool contract-caller) 
    (ok false))))

(define-public (add-pending-miners-to-pool) 
(let ((len-pending-accept-list (len (var-get pending-accept-list))))
  (asserts! (not (is-eq len-pending-accept-list u0)) err-no-pending-miners)
  (asserts! (x-blocks-passed blocks-to-pass) err-more-blocks-to-pass)
  (map add-miner-to-pool (var-get pending-accept-list))
  (asserts! (is-some (as-max-len? (concat (var-get miners-list) (var-get pending-accept-list)) u300)) err-list-length-exceeded)
  (var-set miners-list (unwrap-panic (as-max-len? (concat (var-get miners-list) (var-get pending-accept-list)) u300)))
  (var-set n (+ (var-get n) len-pending-accept-list))
  (var-set pending-accept-list (list ))
  (var-set last-join-done stacks-block-height)
  (some (update-threshold))
  (ok true)))

(define-private (update-threshold) 
(let ((n-now (var-get n))) 
  (if 
    (or 
      (is-eq n-now u1) 
      (is-eq n-now u2)) 
    (var-set k u1)
    (var-set k (/ (* k-percentage (- n-now u1)) u100)))))

(define-private (add-miner-to-pool (miner principal))
(begin 
  (map-delete map-is-pending {address: miner})
  (map-set map-is-miner {address: miner} {value: true})
  (map-set map-block-joined {address: miner} {stacks-block-height: stacks-block-height})
  (ok true)))

(define-private (x-blocks-passed (x uint)) 
(if (>= (- stacks-block-height (var-get last-join-done)) x)
  true
  false))

(define-private (get-k-at-block-asked-to-join (miner-to-vote principal))
(let ((block-asked-to-join (get value (map-get? map-block-asked-to-join {address: miner-to-vote}))))
  (asserts! (is-some block-asked-to-join) err-not-asked-to-join)
  (if 
    (is-eq 
      (unwrap-panic block-asked-to-join) 
      stacks-block-height) 
    (ok (var-get k)) 
    (at-block 
    (unwrap-panic 
      (get-stacks-block-info? id-header-hash 
        (unwrap-panic block-asked-to-join))) 
          (ok (var-get k))))))

(define-private (get-block-number-asked-to-join (miner-to-vote principal))
(let ((block-asked-to-join (get value (map-get? map-block-asked-to-join {address: miner-to-vote}))))
  (if (not (is-some block-asked-to-join)) u0
  (if
    (is-eq
      (unwrap-panic block-asked-to-join)
      stacks-block-height)
    (var-get k)
    (at-block
    (unwrap-panic
      (get-stacks-block-info? id-header-hash
        (unwrap-panic block-asked-to-join)))
          (var-get k))))))

(define-private (get-n-at-block-asked-to-join (miner-to-vote principal)) 
(let ((block-asked-to-join (get value (map-get? map-block-asked-to-join {address: miner-to-vote}))))
  (asserts! (is-some block-asked-to-join) err-not-asked-to-join)
  (if 
    (is-eq 
      (unwrap-panic block-asked-to-join) stacks-block-height) 
    (ok (var-get n)) 
    (at-block  
    (unwrap-panic 
      (get-stacks-block-info? id-header-hash 
        (unwrap-panic block-asked-to-join))) 
          (ok (var-get n))))))

;; LEAVING FLOW

(define-public (leave-pool)
(begin 
  (asserts! (check-is-miner-now contract-caller) err-not-in-miner-map)
  (asserts! (not (is-eq (var-get notifier) contract-caller)) err-currently-notifier)
  (let ((remove-result (unwrap-panic (remove-principal-miners-list contract-caller)))
        (new-k-percentage (if (> (var-get n) u2) (/ (* (var-get k) u100) (- (var-get n) u2)) 
        ;; if n<=2, set a value for new-k-percentage > k-critical to make sure threshold is updated
    (some (var-set miners-list remove-result))
    (var-set n (- (var-get n) u1))
    (map-set map-is-miner {address: contract-caller} {value: false})
    (if 
      (is-some (index-of? (var-get proposed-removal-list) contract-caller)) 
      (var-set proposed-removal-list (unwrap-panic (remove-principal-proposed-removal-list contract-caller))) 
      true)
    (if 
      (>= new-k-percentage k-critical) 
      (if 
        (> (var-get n) u1) 
        (some (update-threshold)) 
        (if 
          (is-eq (var-get n) u1) 
          (some (var-set k u1)) 
          (some (var-set k u0))))
      none)
    (ok true))))

;; REMOVING FLOW

(define-public (propose-removal (miner-to-remove principal))
(begin 
  (asserts! (not (is-eq (var-get n) u1)) err-cant-remove-when-alone-in-pool)
  (asserts! (check-is-miner-now contract-caller) err-not-in-miner-map) 
  (asserts! (check-is-miner-now miner-to-remove) err-not-in-miner-map-miner-to-remove)
  (asserts! (not (check-is-proposed-for-removal-now miner-to-remove)) err-already-proposed-for-removal) 
  (map-set map-block-proposed-to-remove {address: miner-to-remove} {value: stacks-block-height})
  (map-set map-is-proposed-for-removal {address: miner-to-remove} {value: true})
  (var-set proposed-removal-list (unwrap! (as-max-len? (append (var-get proposed-removal-list) miner-to-remove) u300) err-list-length-exceeded))
  (ok true)))

(define-public (vote-positive-remove-request (miner-to-vote principal))
(begin
  (asserts! (not (is-eq contract-caller miner-to-vote)) err-cant-vote-himself)
  (asserts! (check-is-proposed-for-removal-now miner-to-vote) err-not-proposed-for-removal) ;; map_is_proposed_for_removal
  (asserts! (is-ok (get-k-at-block-proposed-removal miner-to-vote)) err-not-proposed-for-removal-proposal-block-missing)
  (asserts! (unwrap! (check-is-miner-when-requested-remove miner-to-vote) err-cant-unwrap-check-miner) err-no-vote-permission)
  (asserts! (not (has-voted-remove miner-to-vote)) err-already-voted) ;; O(1)
  (map-set map-remove-request-voter 
      {miner-to-vote: miner-to-vote, voter: contract-caller} 
      {value: true})
  (if (is-some (get value (map-get? map-votes-accept-removal {address: miner-to-vote}))) 
    (map-set map-votes-accept-removal {address: miner-to-vote} {value: (+ (unwrap-panic (get value (map-get? map-votes-accept-removal  {address: miner-to-vote}))) u1)})
    (map-set map-votes-accept-removal {address: miner-to-vote} {value: u1}))
  (some
    (if (is-vote-accepted (unwrap-panic (get value (map-get? map-votes-accept-removal {address: miner-to-vote}))) (unwrap-panic (get-k-at-block-proposed-removal miner-to-vote)))
      (process-removal miner-to-vote)
      (ok false)))
  (ok true)))

(define-public (vote-negative-remove-request (miner-to-vote principal))
(begin
  (asserts! (not (is-eq contract-caller miner-to-vote)) err-cant-vote-himself)
  (asserts! (check-is-proposed-for-removal-now miner-to-vote) err-not-proposed-for-removal) ;; map_is_waiting
  (asserts! (is-ok (get-k-at-block-proposed-removal miner-to-vote)) err-not-proposed-for-removal-proposal-block-missing)
  (asserts! (unwrap! (check-is-miner-when-requested-remove miner-to-vote) err-cant-unwrap-check-miner) err-no-vote-permission)
  (asserts! (not (has-voted-remove miner-to-vote)) err-already-voted) ;; O(1)
  (map-set map-remove-request-voter 
    {miner-to-vote: miner-to-vote, voter: contract-caller} 
    {value: true})
  (if (is-some (get value (map-get? map-votes-reject-removal {address: miner-to-vote}))) 
    (map-set map-votes-reject-removal {address: miner-to-vote} {value: (+ (unwrap-panic (get value (map-get? map-votes-reject-removal {address: miner-to-vote}))) u1)})
    (map-set map-votes-reject-removal {address: miner-to-vote} {value: u1}))
  (some
    (if (is-vote-rejected-remove (unwrap-panic (get value (map-get? map-votes-reject-removal {address: miner-to-vote}))) (unwrap-panic (get-k-at-block-proposed-removal miner-to-vote)) (unwrap-panic (get-n-at-block-proposed-removal miner-to-vote)))
      (reject-removal miner-to-vote)
      (ok false)))
  (ok true)))

(define-public (quit-proposed-removal-list) 
(begin 
  (asserts! 
    (<= 
      blocks-to-pass
      (- stacks-block-height 
        (default-to stacks-block-height 
          (get value 
            (map-get? map-block-proposed-to-remove {address: contract-caller}))))) 
    err-more-blocks-to-pass)
  (let ((remove-result (unwrap-panic (remove-principal-proposed-removal-list contract-caller))))
    (var-set miner-to-remove-votes-remove contract-caller)
    (var-set proposed-removal-list remove-result)
    (map-delete map-is-proposed-for-removal {address: contract-caller})
    (ok (clear-votes-map-remove-vote contract-caller)))))

(define-private (process-removal (miner principal))
(let ((remove-result (unwrap-panic (remove-principal-miners-list miner)))
      (new-k-percentage (if (> (var-get n) u2) (/ (* (var-get k) u100) (- (var-get n) u2)) u100)))
  (some (var-set miners-list remove-result))
  (var-set miner-to-remove-votes-remove miner)
  (var-set n (- (var-get n) u1))
  (map-delete map-is-miner {address: miner})
  (map-set map-blacklist {address: miner} {value: true})
  (var-set proposed-removal-list (unwrap-panic (remove-principal-proposed-removal-list miner)))
  (clear-votes-map-remove-vote miner)
  (if (>= new-k-percentage k-critical)
    (if 
      (> (var-get n) u1) 
      (update-threshold) 
      (if 
        (is-eq (var-get n) u1) 
        (var-set k u1)
        (var-set k u0)))
    false)
  (ok true)))

(define-private (reject-removal (miner principal))
(begin 
  (var-set miner-to-remove-votes-remove miner)
  (var-set proposed-removal-list (unwrap-panic (remove-principal-proposed-removal-list miner)))
  (clear-votes-map-remove-vote miner)
  (ok true)))

(define-private (has-voted-remove (miner principal)) 
(if (is-some (get value (map-get? map-remove-request-voter {miner-to-vote: miner, voter: contract-caller})))
      (unwrap-panic (get value (map-get? map-remove-request-voter {miner-to-vote: miner, voter: contract-caller})))
      false))

(define-private (clear-votes-map-remove-vote (miner principal)) 
(begin 
  (map-delete map-votes-accept-removal {address: (var-get miner-to-remove-votes-remove)})
  (map-delete map-votes-reject-removal {address: (var-get miner-to-remove-votes-remove)})
  (map-delete map-block-proposed-to-remove {address: (var-get miner-to-remove-votes-remove)})
  (map-delete map-is-proposed-for-removal {address: (var-get miner-to-remove-votes-remove)})
  (map remove-map-record-remove-vote (var-get miners-list))))

(define-private (remove-map-record-remove-vote (miner principal))
(if (is-some (map-get? map-remove-request-voter {miner-to-vote: (var-get miner-to-remove-votes-remove), voter: miner}))
  (map-delete map-remove-request-voter {miner-to-vote: (var-get miner-to-remove-votes-remove), voter: miner})
  false))

;; UPDATE NOTIFIER

(define-public (start-vote-notifier) 
(begin 
  (asserts! (not (var-get notifier-vote-active)) err-vote-started-already)
  (var-set notifier-vote-start-block stacks-block-height)
  (var-set notifier-vote-end-block (+ (var-get notifier-vote-start-block) notifier-election-blocks-to-pass))
  (var-set notifier-vote-active true)
  (if (var-get notifier-previous-entries-removed) 
      (begin 
        (ok (var-set notifier-previous-entries-removed false))) 
      (end-vote-notifier-private))))

(define-public (end-vote-notifier) 
(begin 
  (asserts! (>= stacks-block-height (var-get notifier-vote-end-block)) err-voting-still-active)
  (var-set notifier-vote-active false)
  (end-vote-notifier-private)))

(define-private (end-vote-notifier-private) 
(begin 
  (unwrap! (get-max-votes-number-notifier) (err u99999))
  (if 
    (> 
      (var-get max-votes-notifier) 
      (/ (var-get k) u2)) 
    (var-set notifier (var-get max-voted-proposed-notifier))
    false)
  (delete-all-notifier-entries)
  (ok true)))

(define-private (get-max-votes-number-notifier) 
(ok (map compare-votes-number-notifier (var-get miners-list))))

(define-private (compare-votes-number-notifier (proposed-notifier principal))
  (let ((proposed-notifier-votes (default-to u0 (get votes-number (map-get? map-votes-notifier {voted-notifier: proposed-notifier})))))
  (ok
    (if
      (and
        (> proposed-notifier-votes (/ (var-get k) u2))
        (or
          (> proposed-notifier-votes (var-get max-votes-notifier))
          (and
            (is-eq proposed-notifier-votes (var-get max-votes-notifier))
            (<
              (unwrap-panic (get stacks-block-height (map-get? map-block-joined {address: proposed-notifier})))
              (unwrap-panic (get stacks-block-height (map-get? map-block-joined {address: (var-get max-voted-proposed-notifier)})))
            ))))
      (begin 
        (var-set max-votes-notifier proposed-notifier-votes) 
        (var-set max-voted-proposed-notifier proposed-notifier))
        false))))

(define-private (delete-all-notifier-entries) 
(begin 
  (var-set max-votes-notifier u0)
  (var-set max-voted-proposed-notifier (var-get notifier))
  (map delete-one-notifier-entry (var-get miners-list))
  (var-set notifier-previous-entries-removed true)))

(define-private (delete-one-notifier-entry (miner principal)) 
(begin 
  (map-delete map-voted-update-notifier {miner-who-voted: miner})
  (map-delete map-votes-notifier {voted-notifier: miner})))

(define-public (vote-notifier (voted-notifier principal)) 
(begin 
  (asserts! (and (is-some (get value (map-get? map-is-miner {address: voted-notifier}))) (unwrap-panic (get value (map-get? map-is-miner {address: voted-notifier})))) err-not-in-miner-map)
  (asserts! (and (is-some (get value (map-get? map-is-miner {address: contract-caller}))) (unwrap-panic (get value (map-get? map-is-miner {address: contract-caller})))) err-no-vote-permission)
  (asserts! (var-get notifier-vote-active) err-not-voting-period)
  (asserts! (not (is-eq contract-caller voted-notifier)) err-cant-vote-himself)
  (asserts! (< stacks-block-height (var-get notifier-vote-end-block)) err-not-voting-period)
  (asserts! (is-none (get miner-voted (map-get? map-voted-update-notifier {miner-who-voted: contract-caller}))) err-already-voted)
  (map-set map-voted-update-notifier {miner-who-voted: contract-caller} {miner-voted: voted-notifier})
  (if (is-none (get votes-number (map-get? map-votes-notifier {voted-notifier: voted-notifier}))) 
    (map-set map-votes-notifier {voted-notifier: voted-notifier} {votes-number: u1}) 
    (map-set map-votes-notifier {voted-notifier: voted-notifier} {votes-number: (+ (unwrap-panic (get votes-number (map-get? map-votes-notifier {voted-notifier: voted-notifier}))) u1)}))
  (try! 
    (if (is-vote-accepted (unwrap-panic (get votes-number (map-get? map-votes-notifier {voted-notifier: voted-notifier}))) (var-get k)) 
    (begin 
      (var-set notifier voted-notifier)
      (var-set notifier-vote-end-block stacks-block-height)
      (var-set notifier-vote-active false)
      (end-vote-notifier))
    (ok false)))
(ok true)))

;; WARNING FLOW

(define-public (warn-miner (miner principal)) 
(let ((incremented-value (+ u1 (default-to u0 (get value (map-get? map-warnings {address: miner}))))))
  (asserts! (is-eq contract-caller (var-get notifier)) err-only-notifier) 
  (asserts! 
    (not (and 
      (is-none (at-block (unwrap! (get-stacks-block-info? id-header-hash (- stacks-block-height u1)) err-cant-unwrap-block-info) (get value (map-get? map-warnings {address: miner}))))
      (>= incremented-value u2))) 
    err-one-warning-per-block)
  (asserts! 
    (not 
      (>= 
        (- 
          incremented-value
          (unwrap! (at-block (unwrap! (get-stacks-block-info? id-header-hash (- stacks-block-height u1)) err-cant-unwrap-block-info) (get value (map-get? map-warnings {address: miner}))) err-cant-unwrap-block-info)) 
        u2)) 
    err-one-warning-per-block)
  (ok (map-set map-warnings {address: miner} {value: incremented-value}))))

;; ELECTION FUNCTIONS

(define-private (is-vote-accepted (votes-number uint) (k-local uint))
(if 
  (is-eq k-local u0) ;; k is 0 for n=1, n=2
    (>= votes-number u1) 
    (>= votes-number k-local)))

(define-private (is-vote-rejected-join (votes-number uint) (k-local uint) (n-local uint))
(if 
  (is-eq n-local u1) 
  (>= votes-number u1) 
  (if (is-eq n-local u2) 
    (>= votes-number u2) 
    (>= votes-number (+ (- n-local k-local) u1)))))

(define-private (is-vote-rejected-remove (votes-number uint) (k-local uint) (n-local uint))
(if 
  (is-eq n-local u2) 
  (>= votes-number u1) 
  (if (is-eq n-local u3)
    (>= votes-number u2)
    (>= votes-number (+ (- n-local k-local) u1)))))

;; RECOVERING UNASSIGNED FUNDS FUNCTIONS

;; A public function each miner can call in order to recover the 
;; undistributed rewards caused by dividing rewards to stackers
(define-public (flush-change)
(let ((total-change-funds (var-get reward-change-funds))
      (current-miners-list-len (len (var-get miners-list)))
      (distributed-change-funds (/ total-change-funds current-miners-list-len))
      (change-after-flushing (mod total-change-funds current-miners-list-len))) 
  (asserts! (check-is-miner-now contract-caller) err-not-in-miner-map)
  (var-set temp-distributed-change-funds distributed-change-funds)
  (var-set temp-change-after-flushing change-after-flushing)
  (map flush-distribution-one-miner (var-get miners-list))
  (ok   
    (var-set reward-change-funds 
      (- 
        total-change-funds 
        (* 
          current-miners-list-len 
          distributed-change-funds))))))

(define-private (flush-distribution-one-miner (miner principal))
(let ((miner-balance (default-to u0 (map-get? balance miner)))
      (total-change-funds (var-get reward-change-funds))
      (current-miners-list-len (len (var-get miners-list)))
      (distributed-change-funds (/ total-change-funds current-miners-list-len)))
  (map-set balance miner 
    (+ 
      miner-balance 
      (var-get temp-distributed-change-funds)))))

;; LIST PROCESSING FUNCTIONS

(define-read-only (is-user-accepted) 
(is-vote-accepted
  (default-to u0 
    (get value (map-get? map-votes-accept-join {address: tx-sender})))
    (unwrap-panic (get-k-at-block-asked-to-join tx-sender))))
