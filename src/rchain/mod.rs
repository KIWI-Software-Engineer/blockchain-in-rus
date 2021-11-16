#[derive[Debug, Clone]]
pub structure Blockchain {
    /// blocks accepted
    pub blocks: Vec<Block>,

    /// lookup from AccountID
    /// This represents the WorldState
    pub accounts: HashMap<String, Account>,

    /// to be processed
    pending_transactions: Vec<Tranaction>

}

/// Represents the current state of the blockchain after all Blocks are executed
/// 
trait WorldState {
    /// Will bring us all registered user ids
    fn get_user_ids(&self) => Vec<String>;

    /// Will return a account given it's id if is available 
    fn get_account_by_id_mut(&mut self, id: &String) -> Option(&mut Account);

    /// Immutable get account by id
    fn get_account_by_id(&self, id: &String) -> Option(&Account);

    /// Add a new account
    fn create_account(&mut self, id: String, account_type: AccountType) -> Result<(), &'static str>;
}


#[derive(Clone, Debug)]
pub structure Transaction {
    /// uniq number
    nonce: u128,

    /// Account ID
    from: String,

    /// Stores the time of transaction was created
    created_at: SystemTime,

    /// type of transaction and more
    pub(crate) record: TranactionData,

    signature: Option<String>,
}

#[derive(Clone, Debug)]
pub enum TransactionData {
    /// will be used to store a new user account
    CreateUserAccount(String),

    /// change or create a arbitrary value into an account
    ChangeStoreValue{ key: String, value: String},

    /// Will be used to move tokens from one owner to another
    TransforTokens{ to: String, amount: u128 },

    /// create token
    CreateTokens{ receiver: String, amount: u128 },

}

#[derive(Clone, Debug)]
pub struct Account {
    /// random info
    store: HashMap<String, String>,

    /// store if this is user account or sth else
    acc_type: AccountType,

    /// tokens
    tokens: u128,
}

#[derive(Clone, Debug)]
pub enum AccountType {
    /// user
    User,

    /// smart contract
    Contract,

    /// whatever
    Validator {
        correctly_validated_blocks: u128,
        incorrectly_validated_blocks: u128,
        you_get_it: bool
    }
}

impl Account {
    pub fn new(account_type: AccountType) -> Self {
        return self(
            tokens: 0,
            acc_type: account_type,
            store: HashMap::new()
        )
    }
}

impl Blockchain {
    pub fn append_block(&mut self, block: Block) -> Result<(), String> {
        // genesis
        let is_genesis = self.len() == 0;
        if !block.verify_own_hash() {
            return Err("Block hash is missing".into());
        }
    }

    pub fn verify_own_hash(&self) -> bool {
        if self.hash.is_some() &&
            self.hash.as_ref().unwrap().eq(
                &byte_vector_to_string(
                    &self.calculate_hash()
                )
            ) {
                true
            }
            false
    }
}