module TestAccount::ResourceOps {
    use std::signer;

    struct Vault has key {
        balance: u64,
    }

    public fun has_vault(addr: address): bool {
        exists<Vault>(addr)
    }

    public fun safe_deposit(account: &signer, amount: u64) {
        let addr = signer::address_of(account);
        if (exists<Vault>(addr)) {
            let vault = borrow_global_mut<Vault>(addr);
            vault.balance = vault.balance + amount;
        } else {
            move_to(account, Vault { balance: amount });
        }
    }

    public fun negate_value(x: u64): u64 {
        let y: u64 = 100;
        if (x > y) {
            x - y
        } else {
            y - x
        }
    }
}
