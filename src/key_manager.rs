use crate::util::*;
use ed25519_dalek::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct KeyManager(Keypair);

#[wasm_bindgen]
impl KeyManager {
    /// @description Create an Ed25519 key manager out of seed
    /// @see seedFrom
    #[wasm_bindgen(constructor)]
    pub fn new(seed: &[u8]) -> Result<KeyManager, JsValue> {
        console_error_panic_hook::set_once();

        let secret_key = SecretKey::from_bytes(seed).map_err(into_js_error)?;
        let public_key = PublicKey::from(&secret_key);

        Ok(Self(Keypair {
            secret: secret_key,
            public: public_key,
        }))
    }

    /// @description Get public key bytes
    ///
    /// @example
    /// const keys = new KeyManager();
    /// const publicKey = keys.publicKey();
    #[wasm_bindgen(js_name = publicKey)]
    pub fn public_key(&self) -> Vec<u8> {
        self.0.public.to_bytes()[..].into()
    }

    /// @description Sign message and return signature bytes
    ///
    /// @example
    /// let signature = await keys.sign( message );
    #[wasm_bindgen]
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let signature = self.0.sign(message);
        signature.to_bytes()[..].into()
    }

    /// @description Verify signed message against manager's public key
    ///
    /// @example
    /// let isGenuine = await keys.verify( message, signature );
    #[wasm_bindgen]
    pub fn verify(&self, message: &[u8], signature_bytes: &[u8]) -> Result<bool, JsValue> {
        let signature = Signature::from_bytes(signature_bytes).map_err(into_js_error)?;
        Ok(self.0.verify(message, &signature).is_ok())
    }

    /// @description Verify signed message with provided public key
    ///
    /// @example
    /// let isGenuine = await keys.verifyWithPublicKey( message, signature, publicKey );
    #[wasm_bindgen(js_name = verifyWithPublicKey)]
    pub fn verify_with_public_key(message: &[u8], signature_bytes: &[u8], public_key_bytes: &[u8]) -> Result<bool, JsValue> {
        let public_key = PublicKey::from_bytes(public_key_bytes).map_err(into_js_error)?;
        let signature = Signature::from_bytes(signature_bytes).map_err(into_js_error)?;
        Ok(public_key.verify(message, &signature).is_ok())
    }
}
