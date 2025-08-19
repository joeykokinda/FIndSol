use solana_sdk::signer::{keypair::Keypair, Signer};
use solana_client::rpc_client::RpcClient;

fn main() {
    let mut attempts = 0;
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new  (rpc_url.to_string());
    println!("Searching for bread...\n");
    
    loop {
        let keypair = Keypair::new();
        let pubkey = keypair.pubkey();
        let balance = client.get_balance(&pubkey).unwrap_or(0);


        attempts += 1;
        println!("Checked {} keys ---- ({})", attempts, pubkey );


        if balance > 0 {
            println!("\nYOU FOUND BAL HOLY SHIT {} ! (~{} SOL)", balance, balance as f64 / 1_000_000_000.0);
            println!("   => Wallet Address (Public Key): {}", pubkey);
            println!("   => Private Key (Base58): {}", keypair.to_base58_string());
            println!("\nIt only took {}", attempts);
            break;
        }
    }
}
