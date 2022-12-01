#![cfg(feature = "test-bpf")]
pub mod utils;

use mpl_token_metadata::{
    id, instruction,
    state::{Key, MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URI_LENGTH},
    utils::puffed_out_string,
};
use solana_program::pubkey::Pubkey;
use solana_program_test::*;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use utils::*;

mod mint {

    use mpl_token_metadata::state::{AssetData, Metadata, TokenStandard, PREFIX};
    use solana_program::borsh::try_from_slice_unchecked;

    use super::*;
    #[tokio::test]
    async fn success_mint() {
        let mut context = program_test().start_with_context().await;

        // asset details

        let name = puffed_out_string("Programmable NFT", MAX_NAME_LENGTH);
        let symbol = puffed_out_string("PRG", MAX_SYMBOL_LENGTH);
        let uri = puffed_out_string("uri", MAX_URI_LENGTH);

        let mut asset = AssetData::new(name.clone(), symbol.clone(), uri.clone());
        asset.token_standard = Some(TokenStandard::ProgrammableNonFungible);
        asset.seller_fee_basis_points = 500;
        /*
        asset.programmable_config = Some(ProgrammableConfig {
            rule_set: <PUBKEY>,
        });
        */

        // build the mint transaction

        let token = Keypair::new();
        let mint = Keypair::new();

        let mint_pubkey = mint.pubkey();
        let program_id = id();

        let metadata_seeds = &[PREFIX.as_bytes(), program_id.as_ref(), mint_pubkey.as_ref()];
        let (pubkey, _) = Pubkey::find_program_address(metadata_seeds, &id());

        let payer_pubkey = context.payer.pubkey();
        let mint_ix = instruction::mint(
            /* program id       */ id(),
            /* token account    */ token.pubkey(),
            /* metadata account */ pubkey,
            /* mint account     */ mint.pubkey(),
            /* mint authority   */ payer_pubkey,
            /* payer            */ payer_pubkey,
            /* update authority */ payer_pubkey,
            /* asset data       */ asset,
            /* authority signer */ true,
        );

        let tx = Transaction::new_signed_with_payer(
            &[mint_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await.unwrap();

        // checks the created metadata values

        let metadata_account = get_account(&mut context, &pubkey).await;
        let metadata: Metadata = try_from_slice_unchecked(&metadata_account.data).unwrap();

        assert_eq!(metadata.data.name, name);
        assert_eq!(metadata.data.symbol, symbol);
        assert_eq!(metadata.data.uri, uri);
        assert_eq!(metadata.data.seller_fee_basis_points, 500);
        assert_eq!(metadata.data.creators, None);

        assert!(!metadata.primary_sale_happened);
        assert!(!metadata.is_mutable);
        assert_eq!(metadata.mint, mint_pubkey);
        assert_eq!(metadata.update_authority, context.payer.pubkey());
        assert_eq!(metadata.key, Key::MetadataV1);

        assert_eq!(
            metadata.token_standard,
            Some(TokenStandard::ProgrammableNonFungible)
        );
        assert_eq!(metadata.collection, None);
        assert_eq!(metadata.uses, None);
    }
}
