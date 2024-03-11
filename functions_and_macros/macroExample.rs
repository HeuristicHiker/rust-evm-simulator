// Define the Wallet struct outside of the main function to be globally accessible
struct Wallet {
    owner: String,
    eth_balance: f32,
}

// Define a macro to print the wallet details
macro_rules! print_wallet_details {
    ($wallet:expr) => {
        println!(
            "Wallet owner: {}, Balance: {} ETH",
            $wallet.owner, $wallet.eth_balance
        );
    };
}

fn main() {
    // Initialize a Wallet instance
    let wallet = Wallet {
        owner: "0x123456789...".to_string(),
        eth_balance: 3.5,
    };

    // Use the macro to print the wallet details
    print_wallet_details!(wallet);
}
