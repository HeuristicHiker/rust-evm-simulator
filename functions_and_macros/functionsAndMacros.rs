struct Wallet {
    owner: String,
    eth_balance: f32
}

fn get_wallet_details(wallet: &Wallet, example: String) -> (&str, f32) {
    // takes a wallet which has a string and f32
    // returns the owner and balance

    // TODO Still need to investigate what additional details are traditionally included in transactions
    // TODO look into & descriptor for parameters
    (&wallet.owner, wallet.eth_balance)
}

// Having to put macro_rules! behind functions like this feels weird for now
// I'm 100% going to mess up the order you put params in for awhile
// Still not 100% sure what the benefits are
// I assume it has to do with when/how params are evaluated
macro_rules! print_wallet_details {
    ($wallet:expr) => {
        println!(
            "Wallet owner: {}, Balance: {} ETH",
            $wallet.owner, $wallet.eth_balance
        );
    };
}

fn main() {
    let wallet = Wallet {
        owner: "0x123456789...".to_string(),
        eth_balance: 3.5,
    };

    let macro_wallet = Wallet {
        owner: "0x0987654321...".to_string(),
        eth_balance: 5.0,
    };

    let (owner, balance) = get_wallet_details(&wallet);
    println!("Wallet owner: {}, Balance: {} ETH", owner, balance);

    print_wallet_details!(macro_wallet);
}