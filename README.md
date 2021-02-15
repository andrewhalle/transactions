Processes a CSV of transactions and applies them to accounts.

Designed to be transaction-centric by allowing the process function to take a
mutable reference to state, so that there could be multiple sources of
transactions in the future all sharing a state.

Assumes that all numbers will be formatted with at least one digit after the
decimal (will fail otherwise).
