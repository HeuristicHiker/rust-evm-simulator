// SPDX-License-Identifier: MIT
pragma solidity >=0.8.19;

contract TheMostAmazingContract {
    address public owner;

    // Struct for future use
    struct Wallet {
        string owner;
        uint32 eth_balance;
    }

    mapping(string => uint32) public balances;

    event Transfer(string sender, string receiver, uint32 amount);

    constructor() {
        owner = msg.sender;
        balances["Owner"] = 1000;
    }

    function deposit(string memory act, uint32 amount) public {
        balances[act] += amount;
    }

    function transferBalance(
        string memory sender,
        string memory receiver,
        uint32 amount
    ) public {
        require(
            balances[sender] >= amount,
            "You can't send what you don't have... duh"
        );
        balances[sender] -= amount;
        balances[receiver] += amount;
        emit Transfer(sender, receiver, amount);
    }
}
