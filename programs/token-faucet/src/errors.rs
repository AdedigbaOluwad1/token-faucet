use super::*;

#[error_code]
pub enum Errors {
  #[msg("The faucet is currently inactive")]
  FaucetInactive,
  #[msg("The requested amount exceeds the distribution limit.")]
  AmountExceedsDistributionLimit,
  #[msg("Cooldown time not over")]
  CooldownTimeNotOver
}