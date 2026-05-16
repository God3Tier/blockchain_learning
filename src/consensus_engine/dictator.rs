use ed25519_dalek::{VerifyingKey, Verifier, Signature, Signer, SigningKey};
use crate::{consensus_engine::Consensus, blockchain_features::header::Header};

pub struct Dictator {
    public_key: VerifyingKey, 
    my_signing_key: Option<SigningKey>
}

impl Consensus for Dictator {
    type Digest = Vec<u8>; 

    fn validate(&self, _: &Self::Digest, header: &Header<Self::Digest>) -> bool {
        let signature_bytes: Option<[u8; 64]> = match header.consensus_digest.as_slice().try_into() {
            Ok(bytes) => Some(bytes),
            Err(_) => None
        };

        if signature_bytes.is_none(){
            return false
        }

        let signature = Signature::from(signature_bytes.unwrap());
        
        self.public_key.verify(&header.signature(), &signature).is_ok()
    } 

    fn seal(&self, _: &Self::Digest, partial_header: Header<()>) -> Option<Header<Self::Digest>> {
        let bytes = partial_header.signature();
        if self.my_signing_key.is_none() {
            return None
        }

        let signature = self.my_signing_key.as_ref().unwrap().sign(&bytes).to_string().as_bytes().into();
        
        return Some(Header::new(partial_header.parent.clone(), partial_header.merkle_root, partial_header.state_root, signature, partial_header.block_number))
    }

}