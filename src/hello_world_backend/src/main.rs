use candid::{Principal, CandidType, candid_method};
use std::collections::HashMap;
use std::cell::RefCell;
use serde::{Deserialize};

#[derive(Default, Clone, CandidType, Deserialize)]
pub struct ERC721 {
    name: String,
    symbol: String,
    owners: HashMap<u64, Principal>,
    balances: HashMap<Principal, u64>,
}

#[derive(Default, CandidType, Clone, Deserialize)]
pub struct KYCInfo {
    status : bool,
    update_time : u64,
    expire_time : u64,
}

#[derive(CandidType, Deserialize)]
pub struct Did {
    owner : Principal,
    base_uri : String,
    did_claimed: HashMap<Principal, bool>,
    addr_claimed : HashMap<Principal, bool>,
    tokenid_to_did : HashMap<u64, String>,
    did_to_tokenid : HashMap<String, u64>,
    kyc_map : HashMap<u64, HashMap<Principal, HashMap< u64, KYCInfo>>>,
}

impl Default for Did {
    fn default() -> Self {
        Did {
            owner: Principal::anonymous(),
            base_uri: String::from("https://api.hashkey.id/did/api/nft/metadata/"),
            did_claimed: HashMap::default(),
            addr_claimed: HashMap::default(),
            tokenid_to_did: HashMap::default(),
            did_to_tokenid: HashMap::default(),
            kyc_map: HashMap::default(),
        }
    }
}

thread_local! {
    static ERC721_INSTANCE: RefCell<ERC721> = RefCell::new(ERC721::default());
    static DID_INSTANCE: RefCell<Did> = RefCell::new(Did::default());
}


#[ic_cdk::query]
#[candid_method]
pub fn balance_of(owner: Principal) -> u64 {
    let caller = ic_cdk::caller();
    println!("{}", caller);
    ERC721_INSTANCE.with(|instance| {
        let erc721 = instance.borrow();
        erc721.balances.get(&owner).copied().unwrap_or(0)
    })
}

#[ic_cdk::query]
#[candid_method]
pub fn owner_of(token_id: u64) -> Principal {
    let erc721_instance = ERC721_INSTANCE.with(|instance| instance.borrow().clone());
    let owners = erc721_instance.owners;
    *owners.get(&token_id).unwrap_or(&Principal::anonymous())
}


fn transfer(to: Principal, token_id: u64) {
    ERC721_INSTANCE.with(|instance| {
        let mut erc721 = instance.borrow_mut();
        erc721.owners.insert(token_id, to);
        *erc721.balances.entry(to).or_insert(0) += 1;
    });
}

#[ic_cdk::query]
#[candid_method]
pub fn get_kyc_info(token_id: u64, kyc_provider: Principal, kyc_id: u64) -> KYCInfo {
    DID_INSTANCE.with(|instance| {
        let did = instance.borrow();
        did.kyc_map
            .get(&token_id)
            .and_then(|map| map.get(&kyc_provider))
            .and_then(|inner_map| inner_map.get(&kyc_id))
            .cloned()
            .unwrap_or_else(|| KYCInfo::default())
    })
}

#[ic_cdk::update]
#[candid_method]
pub fn add_kyc(token_id: u64, kyc_provider: Principal, kyc_id: u64, kyc_info: KYCInfo) {
    DID_INSTANCE.with(|instance| {
        let mut did = instance.borrow_mut();
        let kyc_map = did.kyc_map.entry(token_id).or_insert_with(HashMap::new);
        let inner_map = kyc_map.entry(kyc_provider).or_insert_with(HashMap::new);
        inner_map.insert(kyc_id, kyc_info);
    })
}


#[ic_cdk::update]
#[candid_method]
pub fn claim(did: String) {
    assert!(verify_did_format(&did), "invalid name");
    // Check if the DID is already registered
    assert!(!is_did_registered(&did), "DID has already been registered");

    // Check if the address is already registered
    let caller = ic_cdk::caller();
    assert!(!is_address_registered(&caller),"Address has already been registered");

    // Generate a new token ID
    let token_id = generate_token_id(&did);

    // Transfer the NFT to the caller's address
    transfer(caller.clone(), token_id);

    // Update the necessary mappings
    update_did_mappings(&did, token_id);
    update_address_mappings(&caller, token_id);
}

#[ic_cdk::query]
#[candid_method]
pub fn token_id_to_did(token_id: u64) -> Option<String> {
    DID_INSTANCE.with(|instance| {
        let did = instance.borrow();
        did.tokenid_to_did.get(&token_id).cloned()
    })
}

pub fn did_to_token_id(did_str: String) -> Option<u64> {
    DID_INSTANCE.with(|instance| {
        let did = instance.borrow();
        did.did_to_tokenid.get(&did_str).cloned()
    })
}

#[ic_cdk::query]
#[candid_method]
pub fn token_uri(token_id: u64) -> Option<String> {
    DID_INSTANCE.with(|instance| {
        let did = instance.borrow();
        let base_uri = did.base_uri.clone();
        did.tokenid_to_did.get(&token_id).map(|_| format!("{}{}", base_uri, token_id))
    })
}



/* ==============================  UTILS  ============================== */ 
fn is_did_registered(did: &str) -> bool {
    DID_INSTANCE.with(|instance| {
        let did_ref = instance.borrow();
        did_ref.did_to_tokenid.contains_key(did)
    })
}


fn is_address_registered(address: &Principal) -> bool {
    DID_INSTANCE.with(|instance| {
        let did = instance.borrow();
        did.addr_claimed.contains_key(address)
    })
}

fn generate_token_id(_did: &String) -> u64 {
    DID_INSTANCE.with(|instance| {
        let mut erc721 = instance.borrow_mut();
        let next_token_id = erc721.tokenid_to_did.len() as u64;
        erc721.tokenid_to_did.insert(next_token_id, _did.clone());
        next_token_id
    })
}

fn update_did_mappings(did: &str, token_id: u64) {
    DID_INSTANCE.with(|instance| {
        let mut erc721 = instance.borrow_mut();
        erc721.did_to_tokenid.insert(did.to_owned(), token_id);
        erc721.tokenid_to_did.insert(token_id, did.to_owned());
    })
}

fn update_address_mappings(address: &Principal, token_id: u64) {
    DID_INSTANCE.with(|instance| {
        let mut did = instance.borrow_mut();
        did.addr_claimed.insert(address.clone(), true);
        let kyc_info = KYCInfo {
            status: false,
            update_time: 0,
            expire_time: 0,
        };
        let kyc_map = did
            .kyc_map
            .entry(token_id)
            .or_insert_with(HashMap::new);
        let inner_map = kyc_map.entry(address.clone()).or_insert_with(HashMap::new);
        inner_map.insert(0, kyc_info);
    })
}

fn verify_did_format(did: &String) -> bool {
    let b_did = did.as_bytes();

    // length within user [1,50] + .key [4] = [5, 54]
    if b_did.len() < 5 || b_did.len() > 54 {
        return false;
    }

    // allow 0-9/a-z
    for i in 0..b_did.len() - 4 {
        let c = b_did[i];
        if (c < 48 || c > 122) || (c > 57 && c < 97) {
            return false;
        }
    }

    // must end with ".key"
    if b_did[b_did.len() - 4] != 46 || // .
        b_did[b_did.len() - 3] != 107 || // k
        b_did[b_did.len() - 2] != 101 || // e
        b_did[b_did.len() - 1] != 121 // y
    {
        return false;
    }

    true
}


#[ic_cdk::query]
#[candid_method]
pub fn get_base_uri() -> String {
    DID_INSTANCE.with(|instance| {
        let did = instance.borrow();
        did.base_uri.clone()
    })
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    candid::export_service!();
    std::print!("{}", __export_service());
}