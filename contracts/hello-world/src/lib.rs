#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec};

#[contracttype]
#[derive(Clone)]
pub struct RentalAgreement {
    pub agreement_id: u64,
    pub landlord: Address,
    pub tenant: Address,
    pub rent_amount: i128,
    pub duration_months: u64,
    pub is_active: bool,
    pub payments_made: u64,
}

#[contracttype]
pub enum AgreementKey {
    Agreement(u64),
    Count,
}

#[contract]
pub struct RentalAgreementContract;

#[contractimpl]
impl RentalAgreementContract {
    // Register a new rental agreement
    pub fn register_agreement(
        env: Env,
        landlord: Address,
        tenant: Address,
        rent_amount: i128,
        duration_months: u64,
    ) -> u64 {
        let mut count: u64 = env.storage().instance().get(&AgreementKey::Count).unwrap_or(0);
        count += 1;

        let agreement = RentalAgreement {
            agreement_id: count,
            landlord,
            tenant,
            rent_amount,
            duration_months,
            is_active: true,
            payments_made: 0,
        };

        env.storage().instance().set(&AgreementKey::Agreement(count), &agreement);
        env.storage().instance().set(&AgreementKey::Count, &count);

        count
    }

    // Tenant makes a rent payment
    pub fn pay_rent(env: Env, agreement_id: u64, sender: Address) {
        let mut agreement: RentalAgreement =
            env.storage().instance().get(&AgreementKey::Agreement(agreement_id)).expect("Agreement not found");

        if !agreement.is_active {
            panic!("Agreement is not active");
        }

        if sender != agreement.tenant {
            panic!("Only tenant can pay rent");
        }

        agreement.payments_made += 1;

        if agreement.payments_made >= agreement.duration_months {
            agreement.is_active = false; // Automatically close agreement after full term
        }

        env.storage().instance().set(&AgreementKey::Agreement(agreement_id), &agreement);

        // Token transfer logic can be integrated here
    }

    // View a rental agreement
    pub fn view_agreement(env: Env, agreement_id: u64) -> RentalAgreement {
        env.storage().instance().get(&AgreementKey::Agreement(agreement_id)).expect("Agreement not found")
    }
}
