use solana_sdk::signer::{keypair::Keypair, Signer};

fn main() {
    let prefix = "spek";
    let mut attempts = 0;
    
    println!("🔍 Searching for vanity address starting with '{}'...\n", prefix);
    
    loop {
        let keypair = Keypair::new();
        let publickey = keypair.pubkey().to_string();
        
        attempts += 1;
        
        
        println!("Checked {} keys...", attempts);
        
        
        if publickey.starts_with(prefix) {
            println!("\n🎉 Found a match after {} attempts!", attempts);
            println!("   => Wallet Address (Public Key): {}", publickey);
            println!("   => Private Key (Base58): {}", keypair.to_base58_string());
            println!("\nIMPORTANT: Save this private key somewhere safe!");
            break;
        }
    }
}
