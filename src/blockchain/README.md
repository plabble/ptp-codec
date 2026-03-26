# Plabble Blockchain

TODO: write basics about blockchain, transactions etc.


### Using Smart Contracts
1. Upload a BlockData.SmartContract to the memory pool of the Plabble Network
2. Create a transaction that has enough funds to pay for the smart contract (transaction inputs) and has a reference to the smart contract (transaction output)
3. Add a locking script that executes the code of the smart contract, eventually with some parameters
4. Any transaction that uses the smart contract as an input and unlocks it, will be able to execute the code of the smart contract - however, the smart contract will only be unlocked if the execution succeeds, otherwise the transaction will be rejected. This way you can put your checks in the contract itself.

Example transaction:
```toml
version = 1
has_time_lock = false
replaceable_by_fee = false

# Input contains reference to monetary output of previous transaction
[[inputs]]
transaction_id = "010101010101010101010101010101010101010101010101"
output_index = 7

# The script to unlock the previous output (this is not a serious example)
[inputs.unlocking_script]
instructions = ["NOP", "RETURN"]

[[outputs]]
is_monetary = false
burn = false
not_replaceable = true

# The Smart Contract is referenced by its hash/ID here
# this is the same hash as the one in the memory pool (that will end up in the blockchain in the same block, if the transaction is accepted)
[outputs.value]
Asset = "020202020202020202020202020202020202020202020202"

# The locking script calls function #1 of the smart contract and asserts that it returns true
[outputs.locking_script]
instructions = [{ CALL = 1 }, "ASSERT"]

[lock]
Height = 123
```

A unlocking script is always placed BEFORE the locking script and the unlocking script should only contain push opcodes.
A locking script of a smart contract often contains function calls to the smart contract