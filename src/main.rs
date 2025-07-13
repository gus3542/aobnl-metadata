use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
    transaction::Transaction,
    pubkey::Pubkey,
};
use solana_program::{
    system_program,
    sysvar,
};
use mpl_token_metadata::{
    ID as TOKEN_METADATA_PROGRAM_ID,
    instructions::{CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs},
    types::{DataV2, Creator},
};

fn main() {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let payer = Keypair::new(); // 👈 هنا ضع مفتاحك الخاص إذا تريد التوقيع الحقيقي
    let mint_pubkey = Pubkey::new_unique();

    let (metadata_pubkey, _bump) = Pubkey::find_program_address(
        &[
            b"metadata",
            TOKEN_METADATA_PROGRAM_ID.as_ref(),
            mint_pubkey.as_ref(),
        ],
        &TOKEN_METADATA_PROGRAM_ID,
    );

    let data = DataV2 {
        name: "XENOZ Token".to_string(), // 👈 اسم التوكين
        symbol: "XNZ".to_string(),       // 👈 الرمز
        uri: "https://example.com/metadata.json".to_string(), // 👈 رابط metadata
        seller_fee_basis_points: 500, // 👈 5% royalties
        creators: Some(vec![Creator {
            address: payer.pubkey(),
            verified: true,
            share: 100,
        }]),
        collection: None,
        uses: None,
    };

    let args = CreateMetadataAccountV3InstructionArgs {
        data,
        is_mutable: true,
        collection_details: None,
    };

    let ix = CreateMetadataAccountV3 {
        metadata: metadata_pubkey,
        mint: mint_pubkey,
        mint_authority: payer.pubkey(),
        payer: payer.pubkey(),
        update_authority: (payer.pubkey(), true),
        rent: Some(sysvar::rent::ID),
        system_program: system_program::ID,
    }
    .instruction(args);

    let recent_blockhash = client.get_latest_blockhash().expect("Failed to get blockhash");
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let sig = client.send_and_confirm_transaction(&tx).expect("Transaction failed");
    println!("🚀 Transaction Signature: {}", sig);
}

