#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");
extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec
};

use casper_types::{
    contracts::NamedKeys, runtime_args, ApiError, CLType, CLValue,
    ContractHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, Parameter, RuntimeArgs, URef,
};

use casper_contract::{
    contract_api::{
        runtime,
        storage::{self},
    },
    unwrap_or_revert::UnwrapOrRevert,
};

#[repr(u16)]
enum A16ZError{
    InvalidLicenseVersion = 0
}
impl From<A16ZError> for ApiError {
    fn from(e: A16ZError) -> Self {
        ApiError::User(e as u16)
    }
}

const _BASE_LICENSE_URI:&str = "ar://_D9kN1WrNWbCq55BSAGRbTB4bS3v8QAPTYmBThSbX3A/";
const _DEFAULT_BASE_URL:&str = "https://arweave.net/_D9kN1WrNWbCq55BSAGRbTB4bS3v8QAPTYmBThSbX3A/";
const CONTRACT_PACKAGE_HASH:&str = "x16z_contract_package_hash";
const CONTRACT_HASH_KEY_NAME:&str = "contract_hash";


// Only relevant for testing.
/*#[no_mangle]
pub extern "C" fn simulateThirdParty(){
    let _licenseVersion:u64 = 0;
    let contract_hash:ContractHash = runtime::get_named_arg("CONTRACT_HASH");
    let licenseURI:String = runtime::call_contract::<String>(
        contract_hash,
        "getLicenseURI",
        runtime_args!{
            "_licenseVersion" => _licenseVersion
        }
    );
    let a16zlicenses_uref:URef = match runtime::get_key("a16zlicenses"){
        Some(key) => key,
        None => runtime::revert(ApiError::Unhandled)
    }.into_uref().unwrap_or_revert();
    storage::dictionary_put(
        a16zlicenses_uref,
        "test",
        licenseURI,
    );
}*/

#[no_mangle]
pub extern "C" fn getLicenseURI() {
    let licenseVersion:u64 = runtime::get_named_arg("_licenseVersion");
    match licenseVersion {
        0 => licenseVersion,
        1 => licenseVersion,
        2 => licenseVersion,
        3 => licenseVersion,
        4 => licenseVersion,
        5 => licenseVersion,
        _ => runtime::revert(A16ZError::InvalidLicenseVersion)
    };

    let licenseURI:String = String::from(_BASE_LICENSE_URI) + &licenseVersion.to_string();
    let licenseUriCLValue:CLValue = CLValue::from_t(licenseURI).unwrap_or_revert();
    runtime::ret(licenseUriCLValue);
}

#[no_mangle]
pub extern "C" fn getLicenseName(){
    let licenseVersion:u64 = runtime::get_named_arg("_licenseVersion");
    match licenseVersion {
        0 => runtime::ret(CLValue::from_t(String::from("CBE_CC0")).unwrap_or_revert()),
        1 => runtime::ret(CLValue::from_t(String::from("CBE_ECR")).unwrap_or_revert()),
        2 => runtime::ret(CLValue::from_t(String::from("CBE_NECR")).unwrap_or_revert()),
        3 => runtime::ret(CLValue::from_t(String::from("CBE_NECR_HS")).unwrap_or_revert()),
        4 => runtime::ret(CLValue::from_t(String::from("CBE_PR")).unwrap_or_revert()),
        5 => runtime::ret(CLValue::from_t(String::from("CBE_PR_HS")).unwrap_or_revert()),
        _ => runtime::revert(A16ZError::InvalidLicenseVersion)
    };
}

#[no_mangle]
pub extern "C" fn call() {
    let entry_points = {
        let mut entry_points = EntryPoints::new();
        let getLicenseURI = EntryPoint::new(
            "getLicenseURI",
            vec![Parameter::new("_licenseVersion", CLType::U64)],
            CLType::String,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );
        let getLicenseName = EntryPoint::new(
            "getLicenseName",
            vec![Parameter::new("_licenseVersion", CLType::U64)],
            CLType::String,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );
        // Only relevant for testing.
        /*let simulateThirdParty = EntryPoint::new(
            "simulateThirdParty",
            vec![Parameter::new("CONTRACT_HASH", CLType::Key)],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Session,
        );*/
        entry_points.add_entry_point(getLicenseURI);
        entry_points.add_entry_point(getLicenseName);
        // Only relevant for testing.
        //entry_points.add_entry_point(simulateThirdParty);
        entry_points
    };
    let named_keys = {
        let mut named_keys = NamedKeys::new();
        // Only relevant for testing.
        /*let licenses = storage::new_dictionary("a16zlicenses").unwrap_or_revert();
        named_keys.insert("a16zlicenses".to_string(), licenses.into());*/
        named_keys
    };
    storage::new_contract(
        entry_points,
        Some(named_keys),
        Some(String::from(CONTRACT_PACKAGE_HASH)),
        Some(String::from(CONTRACT_HASH_KEY_NAME)),
    );
}
