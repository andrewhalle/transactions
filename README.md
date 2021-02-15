Processes a CSV of transactions and applies them to accounts.

Designed to be transaction-centric by allowing the process function to take a
mutable reference to state, so that there could be multiple sources of
transactions in the future all sharing a state.
