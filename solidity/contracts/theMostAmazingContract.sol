pragma solidity >=0.8.19;

contract TheMostAmazingContract {
    // at some point I'd like to use wallets to see if that's even a good approach but not using yet
    struct Wallet {
        string owner;
        uint32 eth_balance;
    }

    // get balances from owner
    // review why we use int32
    mapping(string => uint32) public balances;

    // Need to use events because ???
    event Transfer(string sender, string receiver, uint32 amount);

    // Not how real transfers work... duh
    // we say it's public since we want anyone to be able to call it
    // Will want to implement owner contract to test and see if we can limit this function later
    function deposit(string memory owner, uint32 amount) public {
        balances[owner] += amount;
    }

    function transferBalance(
        string memory sender,
        string memory receiver,
        uint32 amount
    ) public {
        // Do they have the money
        require(
            balances[sender] >= amount,
            "You can't send what you don't have... duh"
        );

        balances[sender] -= amount;
        balances[receiver] += amount;

        emit Transfer(sender, receiver, amount);
    }
}
