use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
#[cfg(feature = "serde-feature")]
use {
    serde::{Deserialize, Serialize},
    serde_with::{As, DisplayFromStr},
};

use super::InstructionBuilder;
use crate::{
    instruction::MetadataInstruction,
    processor::AuthorizationData,
    state::{
        AssetData, Collection, CollectionDetails, Creator, Data, DataV2, MigrationType,
        ProgrammableConfig, Uses,
    },
};

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum AuthorityType {
    Metadata,
    Delegate,
    Holder,
    Other,
}

//----------------------+
// Instruction args     |
//----------------------+

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
/// Args for create call
pub struct CreateMetadataAccountArgsV3 {
    /// Note that unique metadatas are disabled for now.
    pub data: DataV2,
    /// Whether you want your metadata to be updateable in the future.
    pub is_mutable: bool,
    /// If this is a collection parent NFT.
    pub collection_details: Option<CollectionDetails>,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum CreateArgs {
    V1 {
        asset_data: AssetData,
        decimals: Option<u8>,
        max_supply: Option<u64>,
    },
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum MintArgs {
    V1 {
        amount: u64,
        authorization_data: Option<AuthorizationData>,
    },
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum TransferArgs {
    V1 {
        authorization_data: Option<AuthorizationData>,
        amount: u64,
    },
}

/// Struct representing the values to be updated for an `update` instructions.
///
/// Values that are set to 'None' are not changed; any value set to `Some(_)` will
/// have its value updated. There are properties that have three valid states, which
/// allow the value to remaing the same, to be cleared or to set a new value.
#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum UpdateArgs {
    V1 {
        /// The type of authority requesting the update.
        authority_type: AuthorityType,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
        /// The new update authority.
        new_update_authority: Option<Pubkey>,
        /// The metadata details.
        data: Option<Data>,
        /// Indicates whether the primary sale has happened or not (once set to `true`, it cannot be
        /// changed back).
        primary_sale_happened: Option<bool>,
        // Indicates Whether the data struct is mutable or not (once set to `true`, it cannot be
        /// changed back).
        is_mutable: Option<bool>,
        /// Collection information.
        collection: CollectionToggle,
        /// Additional details of the collection.
        collection_details: CollectionDetailsToggle,
        /// Uses information.
        uses: UsesToggle,
        // Programmable configuration (only applicable to `Programmable` asset types).
        programmable_config: ProgrammableConfigToggle,
    },
}

impl UpdateArgs {
    pub fn get_authority_type(&self) -> AuthorityType {
        match self {
            UpdateArgs::V1 { authority_type, .. } => authority_type.clone(),
        }
    }
}

impl Default for UpdateArgs {
    fn default() -> Self {
        Self::V1 {
            authority_type: AuthorityType::Metadata,
            authorization_data: None,
            new_update_authority: None,
            data: None,
            primary_sale_happened: None,
            is_mutable: None,
            collection: CollectionToggle::None,
            uses: UsesToggle::None,
            collection_details: CollectionDetailsToggle::None,
            programmable_config: ProgrammableConfigToggle::None,
        }
    }
}

//-- Toggle implementations

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum CollectionToggle {
    None,
    Clear,
    Set(Collection),
}

impl CollectionToggle {
    pub fn is_some(&self) -> bool {
        matches!(self, CollectionToggle::Clear | CollectionToggle::Set(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, CollectionToggle::None)
    }

    pub fn is_clear(&self) -> bool {
        matches!(self, CollectionToggle::Clear)
    }

    pub fn is_set(&self) -> bool {
        matches!(self, CollectionToggle::Set(_))
    }

    pub fn to_option(self) -> Option<Collection> {
        match self {
            CollectionToggle::Set(value) => Some(value),
            CollectionToggle::Clear => None,
            CollectionToggle::None => panic!("Tried to convert 'None' value"),
        }
    }
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum UsesToggle {
    None,
    Clear,
    Set(Uses),
}

impl UsesToggle {
    pub fn is_some(&self) -> bool {
        matches!(self, UsesToggle::Clear | UsesToggle::Set(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, UsesToggle::None)
    }

    pub fn is_clear(&self) -> bool {
        matches!(self, UsesToggle::Clear)
    }

    pub fn is_set(&self) -> bool {
        matches!(self, UsesToggle::Set(_))
    }

    pub fn to_option(self) -> Option<Uses> {
        match self {
            UsesToggle::Set(value) => Some(value),
            UsesToggle::Clear => None,
            UsesToggle::None => panic!("Tried to convert 'None' value"),
        }
    }
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum CollectionDetailsToggle {
    None,
    Clear,
    Set(CollectionDetails),
}

impl CollectionDetailsToggle {
    pub fn is_some(&self) -> bool {
        matches!(
            self,
            CollectionDetailsToggle::Clear | CollectionDetailsToggle::Set(_)
        )
    }

    pub fn is_none(&self) -> bool {
        matches!(self, CollectionDetailsToggle::None)
    }

    pub fn is_clear(&self) -> bool {
        matches!(self, CollectionDetailsToggle::Clear)
    }

    pub fn is_set(&self) -> bool {
        matches!(self, CollectionDetailsToggle::Set(_))
    }

    pub fn to_option(self) -> Option<CollectionDetails> {
        match self {
            CollectionDetailsToggle::Set(value) => Some(value),
            CollectionDetailsToggle::Clear => None,
            CollectionDetailsToggle::None => panic!("Tried to convert 'None' value"),
        }
    }
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum ProgrammableConfigToggle {
    None,
    Clear,
    Set(ProgrammableConfig),
}

impl ProgrammableConfigToggle {
    pub fn is_some(&self) -> bool {
        matches!(
            self,
            ProgrammableConfigToggle::Clear | ProgrammableConfigToggle::Set(_)
        )
    }

    pub fn is_none(&self) -> bool {
        matches!(self, ProgrammableConfigToggle::None)
    }

    pub fn is_clear(&self) -> bool {
        matches!(self, ProgrammableConfigToggle::Clear)
    }

    pub fn is_set(&self) -> bool {
        matches!(self, ProgrammableConfigToggle::Set(_))
    }

    pub fn to_option(self) -> Option<ProgrammableConfig> {
        match self {
            ProgrammableConfigToggle::Set(t) => Some(t),
            ProgrammableConfigToggle::Clear => None,
            ProgrammableConfigToggle::None => panic!("Tried to convert 'None' value"),
        }
    }
}

//-- End Toggle implementation

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum MigrateArgs {
    V1 {
        migration_type: MigrationType,
        rule_set: Option<Pubkey>,
    },
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
/// Args for update call
pub struct UpdateMetadataAccountArgsV2 {
    pub data: Option<DataV2>,
    #[cfg_attr(
        feature = "serde-feature",
        serde(with = "As::<Option<DisplayFromStr>>")
    )]
    pub update_authority: Option<Pubkey>,
    pub primary_sale_happened: Option<bool>,
    pub is_mutable: Option<bool>,
}

//----------------------+
// Instruction builders |
//----------------------+

///# Create Metadata Accounts V3 -- Supports v1.3 Collection Details
///
///Create a new Metadata Account
///
/// ### Accounts:
///
///   0. `[writable]` Metadata account
///   1. `[]` Mint account
///   2. `[signer]` Mint authority
///   3. `[signer]` payer
///   4. `[signer]` Update authority
///   5. `[]` System program
///   6. Optional `[]` Rent sysvar
///
/// Creates an CreateMetadataAccounts instruction
#[allow(clippy::too_many_arguments)]
pub fn create_metadata_accounts_v3(
    program_id: Pubkey,
    metadata_account: Pubkey,
    mint: Pubkey,
    mint_authority: Pubkey,
    payer: Pubkey,
    update_authority: Pubkey,
    name: String,
    symbol: String,
    uri: String,
    creators: Option<Vec<Creator>>,
    seller_fee_basis_points: u16,
    update_authority_is_signer: bool,
    is_mutable: bool,
    collection: Option<Collection>,
    uses: Option<Uses>,
    collection_details: Option<CollectionDetails>,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(metadata_account, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(mint_authority, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(update_authority, update_authority_is_signer),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: MetadataInstruction::CreateMetadataAccountV3(CreateMetadataAccountArgsV3 {
            data: DataV2 {
                name,
                symbol,
                uri,
                seller_fee_basis_points,
                creators,
                collection,
                uses,
            },
            is_mutable,
            collection_details,
        })
        .try_to_vec()
        .unwrap(),
    }
}

/// puff metadata account instruction
pub fn puff_metadata_account(program_id: Pubkey, metadata_account: Pubkey) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![AccountMeta::new(metadata_account, false)],
        data: MetadataInstruction::PuffMetadata.try_to_vec().unwrap(),
    }
}

/// Remove Creator Verificaton
#[allow(clippy::too_many_arguments)]
pub fn remove_creator_verification(
    program_id: Pubkey,
    metadata: Pubkey,
    creator: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(metadata, false),
            AccountMeta::new_readonly(creator, true),
        ],
        data: MetadataInstruction::RemoveCreatorVerification
            .try_to_vec()
            .unwrap(),
    }
}

pub fn set_token_standard(
    program_id: Pubkey,
    metadata_account: Pubkey,
    update_authority: Pubkey,
    mint_account: Pubkey,
    edition_account: Option<Pubkey>,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(metadata_account, false),
        AccountMeta::new(update_authority, true),
        AccountMeta::new_readonly(mint_account, false),
    ];
    let data = MetadataInstruction::SetTokenStandard.try_to_vec().unwrap();

    if let Some(edition_account) = edition_account {
        accounts.push(AccountMeta::new_readonly(edition_account, false));
    }

    Instruction {
        program_id,
        accounts,
        data,
    }
}

/// Sign Metadata
#[allow(clippy::too_many_arguments)]
pub fn sign_metadata(program_id: Pubkey, metadata: Pubkey, creator: Pubkey) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(metadata, false),
            AccountMeta::new_readonly(creator, true),
        ],
        data: MetadataInstruction::SignMetadata.try_to_vec().unwrap(),
    }
}

// update metadata account v2 instruction
pub fn update_metadata_accounts_v2(
    program_id: Pubkey,
    metadata_account: Pubkey,
    update_authority: Pubkey,
    new_update_authority: Option<Pubkey>,
    data: Option<DataV2>,
    primary_sale_happened: Option<bool>,
    is_mutable: Option<bool>,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(metadata_account, false),
            AccountMeta::new_readonly(update_authority, true),
        ],
        data: MetadataInstruction::UpdateMetadataAccountV2(UpdateMetadataAccountArgsV2 {
            data,
            update_authority: new_update_authority,
            primary_sale_happened,
            is_mutable,
        })
        .try_to_vec()
        .unwrap(),
    }
}

/// creates a update_primary_sale_happened_via_token instruction
#[allow(clippy::too_many_arguments)]
pub fn update_primary_sale_happened_via_token(
    program_id: Pubkey,
    metadata: Pubkey,
    owner: Pubkey,
    token: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(metadata, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new_readonly(token, false),
        ],
        data: MetadataInstruction::UpdatePrimarySaleHappenedViaToken
            .try_to_vec()
            .unwrap(),
    }
}

//- Instruction Builders trait implementation

/// Builds the instruction to create metadata and associated accounts.
///
/// # Accounts:
///
///   0. `[writable]` Metadata account
///   1. `[optional, writable]` Master edition account
///   2. `[writable]` Mint account
///   3. `[signer]` Mint authority
///   4. `[signer]` Payer
///   5. `[signer]` Update authority
///   6. `[]` System program
///   7. `[]` Instructions sysvar account
///   8. `[]` SPL Token program
impl InstructionBuilder for super::builders::Create {
    fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = vec![
            AccountMeta::new(self.metadata, false),
            // checks whether we have a master edition
            if let Some(master_edition) = self.master_edition {
                AccountMeta::new(master_edition, false)
            } else {
                AccountMeta::new_readonly(crate::ID, false)
            },
            AccountMeta::new(self.mint, self.initialize_mint),
            AccountMeta::new_readonly(self.mint_authority, true),
            AccountMeta::new(self.payer, true),
            AccountMeta::new_readonly(self.update_authority, self.update_authority_as_signer),
            AccountMeta::new_readonly(self.system_program, false),
            AccountMeta::new_readonly(self.sysvar_instructions, false),
            AccountMeta::new_readonly(self.spl_token_program, false),
        ];

        Instruction {
            program_id: crate::ID,
            accounts,
            data: MetadataInstruction::Create(self.args.clone())
                .try_to_vec()
                .unwrap(),
        }
    }
}

impl InstructionBuilder for super::builders::Migrate {
    fn instruction(&self) -> solana_program::instruction::Instruction {
        let mut accounts = vec![
            AccountMeta::new(self.metadata, false),
            AccountMeta::new_readonly(self.edition, false),
            AccountMeta::new(self.token, false),
            AccountMeta::new_readonly(self.mint, false),
            AccountMeta::new(self.payer, true),
            AccountMeta::new_readonly(self.authority, true),
            AccountMeta::new_readonly(self.collection_metadata, false),
            AccountMeta::new_readonly(self.system_program, false),
            AccountMeta::new_readonly(self.sysvar_instructions, false),
            AccountMeta::new_readonly(self.spl_token_program, false),
        ];

        // Optional authorization rules accounts
        if let Some(rules) = &self.authorization_rules {
            accounts.push(AccountMeta::new_readonly(mpl_token_auth_rules::ID, false));
            accounts.push(AccountMeta::new_readonly(*rules, false));
        } else {
            accounts.push(AccountMeta::new_readonly(crate::ID, false));
            accounts.push(AccountMeta::new_readonly(crate::ID, false));
        }

        Instruction {
            program_id: crate::ID,
            accounts,
            data: MetadataInstruction::Migrate(self.args.clone())
                .try_to_vec()
                .unwrap(),
        }
    }
}

/// Mints tokens from a mint account.
///
/// # Accounts:
///
///   0. `[writable]` Token account key
///   1. `[optional]` Owner of the token account
///   2. `[]` Mint of token asset
///   3. `[]` Metadata account key (pda of ['metadata', program id, mint id])")]
///   4. `[optional]` Master Edition account
///   5. `[signer, writable]` Payer
///   6. `[signer]` Authority (mint authority or metadata's update authority for NonFungible asests)
///   7. `[]` System program
///   8. `[]` Instructions sysvar account
///   9. `[]` SPL Token program
///   10. `[]` SPL Associated Token Account program
///   11. `[optional]` Token Authorization Rules program
///   12. `[optional]` Token Authorization Rules account
impl InstructionBuilder for super::builders::Mint {
    fn instruction(&self) -> solana_program::instruction::Instruction {
        let mut accounts = vec![
            AccountMeta::new(self.token, false),
            AccountMeta::new_readonly(self.token_owner.unwrap_or(crate::ID), false),
            AccountMeta::new(self.mint, false),
            AccountMeta::new_readonly(self.metadata, false),
            AccountMeta::new_readonly(self.master_edition.unwrap_or(crate::ID), false),
            AccountMeta::new(self.payer, true),
            AccountMeta::new_readonly(self.authority, true),
            AccountMeta::new_readonly(self.system_program, false),
            AccountMeta::new_readonly(self.sysvar_instructions, false),
            AccountMeta::new_readonly(self.spl_token_program, false),
            AccountMeta::new_readonly(self.spl_ata_program, false),
        ];
        // Optional authorization rules accounts
        if let Some(rules) = &self.authorization_rules {
            accounts.push(AccountMeta::new_readonly(mpl_token_auth_rules::ID, false));
            accounts.push(AccountMeta::new_readonly(*rules, false));
        } else {
            accounts.push(AccountMeta::new_readonly(crate::ID, false));
            accounts.push(AccountMeta::new_readonly(crate::ID, false));
        }

        Instruction {
            program_id: crate::ID,
            accounts,
            data: MetadataInstruction::Mint(self.args.clone())
                .try_to_vec()
                .unwrap(),
        }
    }
}

/// Transfer tokens from a token account.
///
/// # Accounts:
///
///   0. `[signer, writable]` Transfer authority (token or delegate owner)
///   1. `[optional, writable]` Delegate record PDA
///   2. `[writable]` Token account
///   3. `[]` Token account owner")]
///   4. `[writable]` Destination token account
///   5. `[]` Destination token account owner
///   6. `[]` Mint of token asset
///   8. `[writable]` Metadata (pda of ['metadata', program id, mint id])
///   9. `[optional]` Edition of token asset
///   10. `[signer, writable]` Payer
///   11. `[]` SPL Token Program
///   12. `[]` SPL Associated Token Account program
///   13. `[]` System Program
///   14. `[]` Instructions sysvar account
///   15. `[optional]` Token Authorization Rules Program
///   16. `[optional]` Token Authorization Rules account
impl InstructionBuilder for super::builders::Transfer {
    fn instruction(&self) -> solana_program::instruction::Instruction {
        let mut accounts = vec![
            AccountMeta::new(self.authority, true),
            if let Some(delegate_record) = self.delegate_record {
                AccountMeta::new(delegate_record, false)
            } else {
                AccountMeta::new_readonly(crate::ID, false)
            },
            AccountMeta::new(self.token, false),
            AccountMeta::new_readonly(self.token_owner, false),
            AccountMeta::new(self.destination, false),
            AccountMeta::new_readonly(self.destination_owner, false),
            AccountMeta::new_readonly(self.mint, false),
            AccountMeta::new(self.metadata, false),
            AccountMeta::new_readonly(self.edition.unwrap_or(crate::ID), false),
            AccountMeta::new(self.payer, true),
            AccountMeta::new_readonly(self.spl_token_program, false),
            AccountMeta::new_readonly(self.spl_ata_program, false),
            AccountMeta::new_readonly(self.system_program, false),
            AccountMeta::new_readonly(self.sysvar_instructions, false),
        ];
        // Optional authorization rules accounts
        if let Some(rules) = &self.authorization_rules {
            accounts.push(AccountMeta::new_readonly(
                self.authorization_rules_program.unwrap_or(crate::ID),
                false,
            ));
            accounts.push(AccountMeta::new_readonly(*rules, false));
        } else {
            accounts.push(AccountMeta::new_readonly(crate::ID, false));
            accounts.push(AccountMeta::new_readonly(crate::ID, false));
        }

        Instruction {
            program_id: crate::ID,
            accounts,
            data: MetadataInstruction::Transfer(self.args.clone())
                .try_to_vec()
                .unwrap(),
        }
    }
}

impl InstructionBuilder for super::builders::Update {
    fn instruction(&self) -> solana_program::instruction::Instruction {
        let mut accounts = vec![
            AccountMeta::new_readonly(self.authority, true),
            if let Some(record) = self.delegate_record {
                AccountMeta::new(record, false)
            } else {
                AccountMeta::new_readonly(crate::ID, false)
            },
            if let Some(token) = self.token {
                AccountMeta::new(token, false)
            } else {
                AccountMeta::new_readonly(crate::ID, false)
            },
            AccountMeta::new_readonly(self.mint, false),
            AccountMeta::new(self.metadata, false),
            if let Some(edition) = self.edition {
                AccountMeta::new(edition, false)
            } else {
                AccountMeta::new_readonly(crate::ID, false)
            },
            AccountMeta::new_readonly(self.system_program, false),
            AccountMeta::new_readonly(self.sysvar_instructions, false),
        ];

        // Optional authorization rules accounts
        if let Some(rules) = &self.authorization_rules {
            accounts.push(AccountMeta::new_readonly(mpl_token_auth_rules::ID, false));
            accounts.push(AccountMeta::new_readonly(*rules, false));
        } else {
            accounts.push(AccountMeta::new_readonly(crate::ID, false));
            accounts.push(AccountMeta::new_readonly(crate::ID, false));
        }

        Instruction {
            program_id: crate::ID,
            accounts,
            data: MetadataInstruction::Update(self.args.clone())
                .try_to_vec()
                .unwrap(),
        }
    }
}
