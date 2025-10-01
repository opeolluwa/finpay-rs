# Fintech Microservices (Event-Driven Architecture)

## 1. Account Service (Freya)
- **Responsibility:** Manages customer accounts (balance, currency, status).
- **APIs:**
    - `create_account`
    - `get_balance`
    - `list_accounts`
- **Data Ownership:** Owns the **Account DB** (no direct writes from other services).

---

## 2. Transaction Service (Thor)
- **Responsibility:** Handles `deposit`, `withdraw`, `transfer` requests.
- **Events Published:**
    - `TransactionCreated`
    - `TransactionCompleted`
    - `TransactionFailed`
- **Notes:** Ensures **idempotency** (important in financial systems).

---

## 3. FX / Currency Service (Njord)
- **Responsibility:** Stores exchange rates (mocked or fetched from a provider).
- **APIs:**
    - `convert(amount, from_currency, to_currency)`
- **Events Published:**
    - `ExchangeRateUpdated` (e.g., hourly mock refresh).

---

## 4. Ledger Service (Tyr)
- **Responsibility:** Acts as the **source of truth** (double-entry accounting).
- **Events Consumed:**
    - `TransactionCompleted`
- **Actions:** Writes immutable records: debit + credit entries.

---

## 5. Notification Service (Heimdall)
- **Responsibility:** Subscribes to key events.
- **Events Consumed:**
    - `TransactionCompleted`
    - `LowBalance`
- **Actions:** Sends mock emails, logs, or console prints.

---

## 6. User Service (Odin) *(optional)*
- **Responsibility:** Handles user profiles, mock KYC data.
- **Future Extensions:** Can later integrate with authentication & permissions.  
