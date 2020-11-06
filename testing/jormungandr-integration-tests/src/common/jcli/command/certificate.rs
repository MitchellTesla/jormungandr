#![allow(dead_code)]

use jormungandr_lib::interfaces::TaxType;
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub struct CertificateCommand {
    command: Command,
}

impl CertificateCommand {
    pub fn new(command: Command) -> Self {
        Self { command }
    }

    pub fn new_stake_delegation<S: Into<String>, Q: Into<String>>(
        mut self,
        stake_pool_id: S,
        delegation_id: Q,
    ) -> Self {
        self.command
            .arg("new")
            .arg("stake-delegation")
            .arg(delegation_id.into())
            .arg(stake_pool_id.into());
        self
    }

    pub fn retire<S: Into<String>>(mut self, stake_pool_id: S, retirement_time: u64) -> Self {
        self.command
            .arg("new")
            .arg("stake-pool-retirement")
            .arg("--pool-id")
            .arg(stake_pool_id.into())
            .arg("--retirement-time")
            .arg(&retirement_time.to_string());
        self
    }

    pub fn vote<P: AsRef<Path>>(mut self, proposal_file: P) -> Self {
        self.command
            .arg("new")
            .arg("vote-plan")
            .arg(proposal_file.as_ref());
        self
    }

    pub fn vote_tally<S: Into<String>>(mut self, vote_plan_id: S) -> Self {
        self.command
            .arg("new")
            .arg("vote-tally")
            .arg("--vote-plan-id")
            .arg(vote_plan_id.into());
        self
    }

    pub fn public_vote_cast(
        mut self,
        vote_plan_id: String,
        proposal_idx: usize,
        choice: u8,
    ) -> Self {
        self.command
            .arg("new")
            .arg("vote-cast")
            .arg("public")
            .arg("--vote-plan-id")
            .arg(vote_plan_id)
            .arg("--proposal-index")
            .arg(proposal_idx.to_string())
            .arg("--choice")
            .arg(choice.to_string());
        self
    }

    pub fn stake_pool_registration<S: Into<String>, Q: Into<String>, R: Into<String>>(
        mut self,
        kes_key: S,
        vrf_key: Q,
        start_validity: u32,
        management_threshold: u32,
        owner_pk: R,
        tax_type: Option<TaxType>,
    ) -> Self {
        self.command
            .arg("new")
            .arg("stake-pool-registration")
            .arg("--kes-key")
            .arg(kes_key.into())
            .arg("--vrf-key")
            .arg(vrf_key.into())
            .arg("--start-validity")
            .arg(&start_validity.to_string())
            .arg("--management-threshold")
            .arg(&management_threshold.to_string())
            .arg("--owner")
            .arg(owner_pk.into());

        if let Some(tax_type) = tax_type {
            self.command
                .arg("--tax-fixed")
                .arg(tax_type.fixed.to_string())
                .arg("--tax-ratio")
                .arg(format!("{}", tax_type.ratio));

            if let Some(max_limit) = tax_type.max_limit {
                self.command.arg("--tax-limit").arg(max_limit.to_string());
            }
        }
        self
    }

    pub fn stake_pool_id<P: AsRef<Path>, Q: AsRef<Path>>(
        mut self,
        input_file: P,
        output_file: Q,
    ) -> Self {
        self.command
            .arg("show")
            .arg("stake-pool-id")
            .arg("--input")
            .arg(input_file.as_ref())
            .arg("--output")
            .arg(output_file.as_ref());
        self
    }

    pub fn vote_plan_id<P: AsRef<Path>, Q: AsRef<Path>>(
        mut self,
        input_file: P,
        output_file: Q,
    ) -> Self {
        self.command
            .arg("show")
            .arg("vote-plan-id")
            .arg("--input")
            .arg(input_file.as_ref())
            .arg("--output")
            .arg(output_file.as_ref());
        self
    }

    pub fn sign<P: AsRef<Path>, Q: AsRef<Path>, R: AsRef<Path>>(
        mut self,
        signing_key: P,
        input_file: Q,
        output_file: R,
    ) -> Self {
        self.command
            .arg("sign")
            .arg("--key")
            .arg(signing_key.as_ref())
            .arg("--certificate")
            .arg(input_file.as_ref())
            .arg("--output")
            .arg(output_file.as_ref());
        self
    }

    pub fn build(self) -> Command {
        self.command
    }
}
