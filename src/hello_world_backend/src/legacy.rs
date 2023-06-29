use crate::*;

// ======================
//      QUERY  CALLS
// ======================

#[query(manual_reply = true)]
#[candid_method(query)]
fn name() -> ManualReply<Option<String>> {
    dip721_name()
}

#[query(manual_reply = true)]
#[candid_method(query)]
fn logo() -> ManualReply<Option<String>> {
    dip721_logo()
}

#[query(manual_reply = true)]
#[candid_method(query)]
fn symbol() -> ManualReply<Option<String>> {
    dip721_symbol()
}

#[query(manual_reply = true)]
#[candid_method(query)]
fn custodians() -> ManualReply<HashSet<Principal>> {
    dip721_custodians()
}

#[query(manual_reply = true)]
#[candid_method(query)]
fn metadata() -> ManualReply<Metadata> {
    dip721_metadata()
}

#[update(name = "setName", guard = "is_canister_custodian")]
#[candid_method(update, rename = "setName")]
fn set_name(name: String) {
    dip721_set_name(name)
}

#[update(name = "setLogo", guard = "is_canister_custodian")]
#[candid_method(update, rename = "setLogo")]
fn set_logo(logo: String) {
    dip721_set_logo(logo)
}

#[update(name = "setSymbol", guard = "is_canister_custodian")]
#[candid_method(update, rename = "setSymbol")]
fn set_symbol(symbol: String) {
    dip721_set_symbol(symbol)
}
#[update(name = "setCustodians", guard = "is_canister_custodian")]
#[candid_method(update, rename = "setCustodians")]
fn set_custodians(custodians: HashSet<Principal>) {
    dip721_set_custodians(custodians)
}

#[query(name = "totalSupply")]
#[candid_method(query, rename = "totalSupply")]
fn total_supply() -> Nat {
    dip721_total_supply()
}

#[query(name = "totalTransactions")]
#[candid_method(query, rename = "totalTransactions")]
fn total_transactions() -> Nat {
    dip721_total_transactions()
}

#[query()]
#[candid_method(query)]
fn cycles() -> Nat {
    dip721_cycles()
}

#[query(name = "totalUniqueHolders")]
#[candid_method(query, rename = "totalUniqueHolders")]
fn total_unique_holders() -> Nat {
    dip721_total_unique_holders()
}

#[query()]
#[candid_method(query)]
fn stats() -> Stats {
    dip721_stats()
}

#[query(name = "supportedInterfaces")]
#[candid_method(query, rename = "supportedInterfaces")]
fn supported_interfaces() -> Vec<SupportedInterface> {
    dip721_supported_interfaces()
}

#[query(name = "balanceOf")]
#[candid_method(query, rename = "balanceOf")]
fn balance_of(owner: Principal) -> Result<Nat, NftError> {
    dip721_balance_of(owner)
}

#[query(name = "ownerOf")]
#[candid_method(query, rename = "ownerOf")]
fn owner_of(token_identifier: TokenIdentifier) -> Result<Option<Principal>, NftError> {
    dip721_owner_of(token_identifier)
}

#[query(name = "ownerTokenMetadata", manual_reply = true)]
#[candid_method(query, rename = "ownerTokenMetadata")]
fn owner_token_metadata(owner: Principal) -> ManualReply<Result<Vec<TokenMetadata>, NftError>> {
    dip721_owner_token_metadata(owner)
}

#[query(name = "ownerTokenIdentifiers", manual_reply = true)]
#[candid_method(query, rename = "ownerTokenIdentifiers")]
fn owner_token_identifiers(
    owner: Principal,
) -> ManualReply<Result<Vec<TokenIdentifier>, NftError>> {
    dip721_owner_token_identifiers(owner)
}

#[query(name = "tokenMetadata", manual_reply = true)]
#[candid_method(query, rename = "tokenMetadata")]
fn token_metadata(
    token_identifier: TokenIdentifier,
) -> ManualReply<Result<TokenMetadata, NftError>> {
    dip721_token_metadata(token_identifier)
}

//#[update(name = "mint", guard = "is_canister_custodian")]
#[update(name = "mint")]
#[candid_method(update, rename = "mint")]
fn mint(
    to: Principal,
    properties: Vec<(String, GenericValue)>,
) -> Result<Nat, NftError> {
    let token_identifier = dip721_total_supply();
    dip721_mint(to, token_identifier + 1, properties)
}
