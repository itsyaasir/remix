use candid::CandidType;
use candid::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, CandidType)]
pub enum RemixError {}
