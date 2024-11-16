use std::collections::HashMap;

use alloy::{primitives::Address, signers::local::PrivateKeySigner};

pub trait SignerRegistry<Index: Ord> {
    fn get_signer(&self, idx: Index) -> Option<&PrivateKeySigner>;
    fn get_address(&self, idx: Index) -> Option<Address>;
}

pub trait AgentRegistry<Index: Ord> {
    fn get_agent(&self, idx: Index) -> Option<&Address>;
}

pub struct SignerStore {
    pub signers: Vec<PrivateKeySigner>,
}

pub struct AgentStore {
    agents: HashMap<String, SignerStore>,
}

impl Default for AgentStore {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentStore {
    pub fn new() -> Self {
        AgentStore {
            agents: HashMap::new(),
        }
    }

    pub fn add_agent(&mut self, name: impl AsRef<str>, signers: SignerStore) {
        self.agents.insert(name.as_ref().to_owned(), signers);
    }

    pub fn add_random_agent(&mut self, name: impl AsRef<str>, num_signers: usize) {
        let signers = SignerStore::new_random(num_signers);
        self.add_agent(name, signers);
    }

    pub fn get_agent(&self, name: impl AsRef<str>) -> Option<&SignerStore> {
        self.agents.get(name.as_ref())
    }

    pub fn all_agents(&self) -> impl Iterator<Item = (&String, &SignerStore)> {
        self.agents.iter()
    }

    pub fn has_agent(&self, name: impl AsRef<str>) -> bool {
        self.agents.contains_key(name.as_ref())
    }

    pub fn remove_agent(&mut self, name: impl AsRef<str>) {
        self.agents.remove(name.as_ref());
    }
}

impl<Idx> SignerRegistry<Idx> for SignerStore
where
    Idx: Ord + Into<usize>,
{
    fn get_signer(&self, idx: Idx) -> Option<&PrivateKeySigner> {
        self.signers.get::<usize>(idx.into())
    }

    fn get_address(&self, idx: Idx) -> Option<Address> {
        self.signers.get::<usize>(idx.into()).map(|s| s.address())
    }
}

impl SignerStore {
    pub fn new() -> Self {
        SignerStore {
            signers: Vec::new(),
        }
    }

    pub fn new_random(num_signers: usize) -> Self {
        let signers: Vec<PrivateKeySigner> = (0..num_signers)
            .map(|_| PrivateKeySigner::random())
            .collect();
        SignerStore { signers }
    }

    pub fn add_signer(&mut self, signer: PrivateKeySigner) {
        self.signers.push(signer);
    }

    pub fn remove_signer(&mut self, idx: usize) {
        self.signers.remove(idx);
    }
}
