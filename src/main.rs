use web3::contract::{Contract, Options};
use web3::types::{Address, TransactionReceipt};

fn main() {
    // Create a new Web3 client.
    let web3 = web3::Web3::new("http://localhost:8545");

    // Get the address of the chat contract.
    let chat_address = web3.eth().accounts().get(0).unwrap();

    // Create a new instance of the chat contract.
    let chat = Contract::new(chat_address, web3);

    // Get the current list of users.
    let users = chat.users().call().unwrap();

    // Send a message to the user with the address "0x1234567890abcdef".
    let message = "Hello, world!";
    let receipt = chat
        .send_message(message, "0x1234567890abcdef")
        .send()
        .unwrap();

    // Check the status of the transaction receipt.
    if receipt.status() == true {
        println!("Message sent successfully!");
    } else {
        println!("Error sending message: {}", receipt.error());
    }
}
