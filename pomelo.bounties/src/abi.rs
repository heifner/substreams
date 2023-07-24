// Generated by antelope-abi2rs 0.4.0 - eosio::abi/1.2

use serde::{Deserialize, Deserializer, Serialize};

type Asset = String;
type Name = String;
type Checksum256 = String;
type Symbol = String;
type SymbolCode = String;
type TimePointSec = String;
type Uint32 = u32;
type Uint64 = u64;
type Float64 = String;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ExtendedAsset {
    pub quantity: Asset,
    pub contract: Name,
}

fn str_or_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StrOrU64<'a> {
        Str(&'a str),
        U64(u64),
    }

    Ok(match StrOrU64::deserialize(deserializer)? {
        StrOrU64::Str(v) => v
            .parse::<u64>()
            .map_err(|_| serde::de::Error::custom("failed to parse u64 number"))?,
        StrOrU64::U64(v) => v,
    })
}

macro_rules! impl_try_from_str {
    ($type:ty) => {
        impl TryFrom<&str> for $type {
            type Error = serde_json::Error;
            #[inline]
            fn try_from(str: &str) -> Result<Self, Self::Error> {
                serde_json::from_str(str)
            }
        }
    };
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Apply {
    pub bounty_id: Name,
    pub user_id: Name,
}
impl_try_from_str!(Apply);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Approve {
    pub bounty_id: Name,
    pub applicant_user_id: Name,
}
impl_try_from_str!(Approve);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BountiesRow {
    pub bounty_id: Name,
    pub author_user_id: Name,
    pub funders: Vec<PairNameAsset>,
    pub amount: ExtendedAsset,
    pub fee: ExtendedAsset,
    pub claimed: Asset,
    pub applicant_user_ids: Vec<Name>,
    pub approved_user_id: Name,
    pub status: Name,
    pub r#type: Name,
    pub permissions: Name,
    pub metadata: Vec<PairNameString>,
    pub created_at: TimePointSec,
    pub updated_at: TimePointSec,
    pub submitted_at: TimePointSec,
    pub completed_at: TimePointSec,
}
impl_try_from_str!(BountiesRow);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Claim {
    pub bounty_id: Name,
    pub receiver: Name,
}
impl_try_from_str!(Claim);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Claimlog {
    pub bounty_id: Name,
    pub receiver: Name,
    pub ext_quantity: ExtendedAsset,
    pub fee: Asset,
    pub status: Name,
    pub worker_user_id: Name,
    pub days_since_created: Uint32,
}
impl_try_from_str!(Claimlog);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Close {
    pub bounty_id: Name,
}
impl_try_from_str!(Close);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Complete {
    pub bounty_id: Name,
}
impl_try_from_str!(Complete);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ConfigsRow {
    pub status: Name,
    #[serde(deserialize_with = "str_or_u64")]
    pub fee: Uint64,
    pub login_contract: Name,
    pub fee_account: Name,
    pub metadata_keys: Vec<Name>,
}
impl_try_from_str!(ConfigsRow);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Create {
    pub author_user_id: Name,
    pub bounty_id: Name,
    pub accepted_token: SymbolCode,
    pub bounty_type: Option<Name>,
}
impl_try_from_str!(Create);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Createlog {
    pub bounty_id: Name,
    pub author_user_id: Name,
    pub ext_sym: ExtendedSymbol,
    pub r#type: Name,
    pub permissions: Name,
}
impl_try_from_str!(Createlog);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Deltoken {
    pub symcode: SymbolCode,
}
impl_try_from_str!(Deltoken);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Deny {
    pub bounty_id: Name,
}
impl_try_from_str!(Deny);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Depositlog {
    pub bounty_id: Name,
    pub funder_user_id: Name,
    pub from: Name,
    pub ext_quantity: ExtendedAsset,
    pub fee: Asset,
    pub value: Float64,
    pub memo: String,
}
impl_try_from_str!(Depositlog);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ExtendedSymbol {
    pub sym: Symbol,
    pub contract: Name,
}
impl_try_from_str!(ExtendedSymbol);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Forfeit {
    pub bounty_id: Name,
}
impl_try_from_str!(Forfeit);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PairNameAsset {
    pub first: Name,
    pub second: Asset,
}
impl_try_from_str!(PairNameAsset);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PairNameString {
    pub first: Name,
    pub second: String,
}
impl_try_from_str!(PairNameString);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Publish {
    pub bounty_id: Name,
}
impl_try_from_str!(Publish);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Release {
    pub bounty_id: Name,
}
impl_try_from_str!(Release);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Setconfig {
    pub status: Option<Name>,
    pub fee: Option<Uint64>,
    pub login_contract: Option<Name>,
    pub fee_account: Option<Name>,
    pub metadata_keys: Vec<Name>,
}
impl_try_from_str!(Setconfig);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Setmetadata {
    pub bounty_id: Name,
    pub metadata_key: Name,
    pub metadata_value: String,
}
impl_try_from_str!(Setmetadata);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Setstate {
    pub bounty_id: Name,
    pub state: Name,
}
impl_try_from_str!(Setstate);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Statelog {
    pub bounty_id: Name,
    pub status: Name,
    pub action: Name,
}
impl_try_from_str!(Statelog);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StatusRow {
    pub counters: Vec<Uint32>,
    pub last_updated: TimePointSec,
}
impl_try_from_str!(StatusRow);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Syncbounty {
    pub bounty_id: Name,
    pub state: Name,
    pub applicant_user_ids: Vec<Name>,
    pub approved_user_id: Option<Name>,
    pub updated_at: TimePointSec,
    pub submitted_at: Option<TimePointSec>,
    pub completed_at: Option<TimePointSec>,
}
impl_try_from_str!(Syncbounty);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Token {
    pub sym: Symbol,
    pub contract: Name,
    #[serde(deserialize_with = "str_or_u64")]
    pub min_amount: Uint64,
    #[serde(deserialize_with = "str_or_u64")]
    pub oracle_id: Uint64,
}
impl_try_from_str!(Token);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TokensRow {
    pub sym: Symbol,
    pub contract: Name,
    #[serde(deserialize_with = "str_or_u64")]
    pub min_amount: Uint64,
    #[serde(deserialize_with = "str_or_u64")]
    pub oracle_id: Uint64,
}
impl_try_from_str!(TokensRow);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TransfersRow {
    #[serde(deserialize_with = "str_or_u64")]
    pub transfer_id: Uint64,
    pub bounty_id: Name,
    pub funder_user_id: Name,
    pub from: Name,
    pub to: Name,
    pub ext_quantity: ExtendedAsset,
    pub fee: Asset,
    pub memo: String,
    pub value: Float64,
    pub trx_id: Checksum256,
    pub created_at: TimePointSec,
}
impl_try_from_str!(TransfersRow);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Withdraw {
    pub bounty_id: Name,
    pub receiver: Name,
}
impl_try_from_str!(Withdraw);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Withdrawlog {
    pub bounty_id: Name,
    pub status: Name,
    pub author_user_id: Name,
    pub receiver: Name,
    pub refund: ExtendedAsset,
}
impl_try_from_str!(Withdrawlog);

