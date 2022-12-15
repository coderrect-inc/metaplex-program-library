use mpl_token_metadata::{
    id, instruction,
    instruction::MintArgs,
    state::{
        AssetData, Creator, Metadata, ProgrammableConfig, TokenMetadataAccount, TokenStandard,
        EDITION, PREFIX,
    },
};
use solana_program::pubkey::Pubkey;
use solana_program_test::{BanksClientError, ProgramTestContext};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

pub const DEFAULT_NAME: &str = "Digital Asset";
pub const DEFAULT_SYMBOL: &str = "DA";
pub const DEFAULT_URI: &str = "https://digital.asset.org";

pub struct DigitalAsset {
    pub metadata: Pubkey,
    pub mint: Keypair,
    pub token: Option<Pubkey>,
    pub master_edition: Option<Pubkey>,
}

impl Default for DigitalAsset {
    fn default() -> Self {
        Self::new()
    }
}

impl DigitalAsset {
    pub fn new() -> Self {
        let mint = Keypair::new();
        let mint_pubkey = mint.pubkey();
        let program_id = id();

        let metadata_seeds = &[PREFIX.as_bytes(), program_id.as_ref(), mint_pubkey.as_ref()];
        let (metadata, _) = Pubkey::find_program_address(metadata_seeds, &program_id);

        Self {
            metadata,
            mint,
            token: None,
            master_edition: None,
        }
    }

    pub async fn create(
        &mut self,
        context: &mut ProgramTestContext,
        token_standard: TokenStandard,
        authorization_rules: Option<Pubkey>,
    ) -> Result<(), BanksClientError> {
        let mut asset = AssetData::new(
            token_standard,
            String::from(DEFAULT_NAME),
            String::from(DEFAULT_SYMBOL),
            String::from(DEFAULT_URI),
            context.payer.pubkey(),
        );
        asset.seller_fee_basis_points = 500;

        let creators = vec![Creator {
            address: context.payer.pubkey(),
            share: 100,
            verified: true,
        }];
        asset.creators = Some(creators);

        if let Some(authorization_rules) = authorization_rules {
            asset.programmable_config = Some(ProgrammableConfig {
                rule_set: authorization_rules,
            });
        }

        let payer_pubkey = context.payer.pubkey();
        let mint_pubkey = self.mint.pubkey();

        let program_id = id();

        let master_edition = match token_standard {
            TokenStandard::NonFungible | TokenStandard::ProgrammableNonFungible => {
                // master edition PDA address
                let master_edition_seeds = &[
                    PREFIX.as_bytes(),
                    program_id.as_ref(),
                    mint_pubkey.as_ref(),
                    EDITION.as_bytes(),
                ];
                let (master_edition, _) = Pubkey::find_program_address(master_edition_seeds, &id());
                Some(master_edition)
            }
            _ => None,
        };

        let create_ix = instruction::create(
            /* metadata account */ self.metadata,
            /* master edition   */ master_edition,
            /* mint account     */ self.mint.pubkey(),
            /* mint authority   */ payer_pubkey,
            /* payer            */ payer_pubkey,
            /* update authority */ payer_pubkey,
            /* initialize mint  */ true,
            /* authority signer */ true,
            /* asset data       */ asset,
            /* decimals         */ Some(0),
            /* max supply       */ Some(0),
        );

        let tx = Transaction::new_signed_with_payer(
            &[create_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &self.mint],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await.unwrap();

        self.master_edition = master_edition;

        Ok(())
    }

    pub async fn mint(
        &mut self,
        context: &mut ProgramTestContext,
        authorization_rules: Option<Pubkey>,
        amount: u64,
    ) -> Result<(), BanksClientError> {
        let payer_pubkey = context.payer.pubkey();
        let (token, _) = Pubkey::find_program_address(
            &[
                &payer_pubkey.to_bytes(),
                &spl_token::id().to_bytes(),
                &self.mint.pubkey().to_bytes(),
            ],
            &spl_associated_token_account::id(),
        );

        let mint_ix = instruction::mint(
            /* token account       */ token,
            /* metadata account    */ self.metadata,
            /* mint account        */ self.mint.pubkey(),
            /* payer               */ payer_pubkey,
            /* authority           */ payer_pubkey,
            /* master edition      */ self.master_edition,
            /* authorization rules */ authorization_rules,
            /* amount              */ MintArgs::V1 { amount },
        );

        let tx = Transaction::new_signed_with_payer(
            &[mint_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        );

        match context.banks_client.process_transaction(tx).await {
            Ok(_) => {
                self.token = Some(token);
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    pub async fn create_and_mint(
        &mut self,
        context: &mut ProgramTestContext,
        token_standard: TokenStandard,
        authorization_rules: Option<Pubkey>,
        amount: u64,
    ) -> Result<(), BanksClientError> {
        // creates the metadata
        self.create(context, token_standard, authorization_rules)
            .await
            .unwrap();
        // mints tokens
        self.mint(context, authorization_rules, amount).await
    }

    pub async fn get_metadata(&self, context: &mut ProgramTestContext) -> Metadata {
        let metadata_account = context
            .banks_client
            .get_account(self.metadata)
            .await
            .unwrap()
            .unwrap();

        let metadata = Metadata::safe_deserialize(&metadata_account.data).unwrap();

        metadata
    }

    pub async fn get_asset_data(&self, context: &mut ProgramTestContext) -> AssetData {
        let metadata = self.get_metadata(context).await;

        metadata.into_asset_data()
    }

    pub async fn compare_asset_data(
        &self,
        context: &mut ProgramTestContext,
        asset_data: &AssetData,
    ) {
        let on_chain_asset_data = self.get_asset_data(context).await;

        assert_eq!(on_chain_asset_data, *asset_data);
    }
}
