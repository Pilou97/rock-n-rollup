use serde::de::DeserializeOwned;

use crate::{
    core::{FromInput, FromRawInput},
    plugins::{
        crypto::{PublicKey, Signature, Verifier},
        database::Database,
        hasher::Hasher,
    },
};

/// The idea is to have a common operation that represent external message
///
/// Here is the encoding of this operation:
/// ------------
/// 0x01\n
/// rollup-address\n
/// public_key\n
/// nonce\n
/// signature\n
/// payload
///------------
///
/// The signature represent the signature of `{nonce}{rollup-address}{hash of the payload}`
///
/// A typescript library should be provided to ease the development of rollup

#[derive(Clone)]
pub struct RawExternalOperation {
    destination: String,
    public_key: PublicKey,
    nonce: u64,
    signature: Signature,
    payload: Vec<u8>,
}

/// Convert some bytes to an external operation
///
/// Should be private because it does not verify the signature
fn try_from_bytes(value: &[u8]) -> Result<RawExternalOperation, ()> {
    if !value.starts_with(&[1]) {
        return Err(());
    }
    let value = value.iter().skip(1).copied().collect::<Vec<u8>>();

    let mut iter = value.split(|byte| byte == &0xA);
    let destination = iter.next().map(|bytes| bytes.to_vec());
    let public_key = iter.next().map(|bytes| bytes.to_vec());
    let nonce = iter.next().map(|bytes| bytes.to_vec());
    let signature = iter.next().map(|bytes| bytes.to_vec());
    let payload = iter
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<u8>>>()
        .join(&0xA);

    match (destination, public_key, nonce, signature, payload) {
        (Some(destination), Some(public_key), Some(nonce), Some(signature), payload) => {
            let destination = String::from_utf8(destination).map_err(|_| ())?;

            let public_key = String::from_utf8(public_key).map_err(|_| ())?;
            let public_key = PublicKey::try_from(public_key)?;

            let nonce = u64::from_be_bytes(nonce.try_into().map_err(|_| ())?);

            let signature = String::from_utf8(signature).map_err(|_| ())?;
            let signature = Signature::try_from(signature)?;

            Ok(RawExternalOperation {
                destination,
                public_key,
                nonce,
                signature,
                payload,
            })
        }
        _ => Err(()),
    }
}

impl FromRawInput for RawExternalOperation {
    fn from_raw_input<R: Database + Hasher + Verifier>(
        runtime: &mut R,
        raw_input: &crate::core::RawInput,
    ) -> Result<Self, ()> {
        let unverified = try_from_bytes(&raw_input.payload)?;

        // TODO: I would prefer to save the address and not the public key
        let user_nonce_path = format!("/account-nonce/{}", unverified.public_key.to_string());
        let user_nonce = runtime
            .get::<u64>(&user_nonce_path)
            .map_err(|_| ())?
            .unwrap_or_default();

        let operation_nonce = unverified.nonce;

        if operation_nonce != user_nonce + 1 {
            return Err(());
        }

        // Verifying the signature

        let hash = {
            let mut nonce = unverified.nonce.to_be_bytes().to_vec();
            let mut address = unverified.destination.as_bytes().to_vec();
            let mut payload_hash = runtime.hash_512(&unverified.payload).as_ref().to_vec();
            let mut to_hash = Vec::default();
            to_hash.append(&mut nonce);
            to_hash.append(&mut address);
            to_hash.append(&mut payload_hash);

            runtime.hash_512(&to_hash)
        };
        let signature = &unverified.signature;
        let public_key = &unverified.public_key;
        runtime.verify_signature(signature, public_key, hash.as_ref());

        // Save the new nonce if the signature is correct
        let _ = runtime.save(&user_nonce_path, &operation_nonce);

        // And then the operation is a verified one
        Ok(unverified)
    }
}

///////// More convenient type

pub struct Json<P>
where
    P: DeserializeOwned,
{
    pub payload: P,
}

impl<P, S> FromInput<RawExternalOperation, S> for Json<P>
where
    P: DeserializeOwned,
{
    fn from_input<R: crate::core::Runtime>(
        _: &mut R,
        input: &crate::core::Input<RawExternalOperation>,
        _: &S,
    ) -> Result<Self, ()> {
        // TODO: find a better serialization protocol;
        let bytes = (&input.payload.payload).clone();
        let string = String::from_utf8(bytes).map_err(|_| ())?;
        let payload = serde_json_wasm::from_str::<P>(&string).map_err(|_| ())?;
        Ok(Json { payload })
    }
}

#[cfg(test)]
mod tests {

    use serde::Deserialize;

    use crate::core::{MockRuntime, Runtime, Service};

    use super::{try_from_bytes, Json, RawExternalOperation};

    fn transition<R: Runtime>(rt: &mut R, _: RawExternalOperation) {
        rt.write_debug("Hello {source}")
    }

    #[derive(Deserialize)]
    enum PingPong {
        Ping,
        Pong,
    }

    fn transition_2<R: Runtime>(_: &mut R, msg: Json<PingPong>) {
        let _msg = msg.payload;
    }

    #[test]
    fn test() {
        let mut service = Service::<MockRuntime, RawExternalOperation, ()>::new(());
        service.register(transition).register(transition_2);
    }

    #[test]
    fn deserialization() {
        let destination = "src13mudsG5iD2E7UqzkWbHR1yPkAqmrmtD9NXc57agXVM8zMbxnbq";
        let public_key = "edpkuDMUm7Y53wp4gxeLBXuiAhXZrLn8XB1R83ksvvesH8Lp8bmCfK";
        let nonce: u64 = 1;
        let signature = "edsigtuU5nUqBniorqTFXFixkG6ZkfvEPrfc9aT9DnMAeims2AX2yjpgYaedXBoKzAGHE3ZXSi1hZz6piZ3itTE7f2F4FoaxXtM";
        let payload = vec![0xA, 0x01, 0x01, 0xA, 0x1, 0x1, 0xA];
        let message = {
            let mut msg = vec![0x1];

            let mut destination = destination.as_bytes().to_vec();
            let mut public_key = public_key.as_bytes().to_vec();
            let mut nonce = nonce.to_be_bytes().to_vec();
            let mut signature = signature.as_bytes().to_vec();
            let mut payload = payload.clone();

            msg.append(&mut destination);
            msg.push(0xA);

            msg.append(&mut public_key);
            msg.push(0xA);

            msg.append(&mut nonce);
            msg.push(0xA);

            msg.append(&mut signature);
            msg.push(0xA);

            msg.append(&mut payload);

            msg
        };

        let msg = try_from_bytes(&message);

        assert!(msg.is_ok());
        let msg = msg.unwrap();

        assert_eq!(msg.destination, destination);
        // assert_eq!(msg.public_key, public_key);
        assert_eq!(msg.nonce, nonce);
        // assert_eq!(msg.signature, signature);
        assert_eq!(msg.payload, payload);
    }
}
