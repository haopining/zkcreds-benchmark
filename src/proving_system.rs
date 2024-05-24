use rand::Rng;

pub trait ZeroKnowledgeProof {
    type ProvingKey;
    type VerifyingKey;
    type Proof;
    type PublicInput;
    type PrivateInput;
    type Error;

    fn setup<R: Rng>(rng: &mut R) -> Result<(Self::ProvingKey, Self::VerifyingKey), Self::Error>;
    fn prove<R: Rng>(
        rng: &mut R,
        proving_key: &Self::ProvingKey,
        public_input: &Self::PublicInput,
        private_input: &Self::PrivateInput,
    ) -> Result<Self::Proof, Self::Error>;
    fn verify(
        verifying_key: &Self::VerifyingKey,
        public_input: &Self::PublicInput,
        proof: &Self::Proof,
    ) -> Result<bool, Self::Error>;
}
