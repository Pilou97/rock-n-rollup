use crate::core::FromRawInput;

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
pub struct ExternalOperation {
    destination: String,
    public_key: String,
    nonce: u64,
    signature: String,
    payload: Vec<u8>,
}

/// Convert some bytes to an external operation
///
/// Should be private because it does not verify the signature
fn try_from_bytes(value: &[u8]) -> Result<ExternalOperation, ()> {
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
            let nonce = u64::from_be_bytes(nonce.try_into().map_err(|_| ())?);
            let signature = String::from_utf8(signature).map_err(|_| ())?;

            Ok(ExternalOperation {
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

impl FromRawInput for ExternalOperation {
    fn from_raw_input<R: crate::core::Runtime>(
        _: &mut R,
        raw_input: &crate::core::RawInput,
    ) -> Result<Self, ()> {
        let unverified = try_from_bytes(&raw_input.payload)?;
        let op_nonce = unverified.nonce;
        // TODO: get the nonce
        let user_nonce = Ok(1)?;

        if op_nonce != user_nonce + 1 {
            return Err(());
        }

        // TODO: save this
        let _user_nonce = op_nonce;

        // TODO: verify signature

        Ok(unverified)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{Application, MockRuntime, Runtime, Service};

    use super::{try_from_bytes, ExternalOperation};

    fn transition<R: Runtime>(rt: &mut R, _: ExternalOperation) {
        rt.write_debug("Hello {source}")
    }

    #[test]
    fn test() {
        let mut runtime = MockRuntime::default();

        let mut service = Service::<MockRuntime, ExternalOperation>::default();
        let mut application = Application::new(&mut runtime);

        service.register(transition);

        application.service(service);
    }

    #[test]
    fn deserialization() {
        let destination = "src13mudsG5iD2E7UqzkWbHR1yPkAqmrmtD9NXc57agXVM8zMbxnbq";
        let public_key = "edpkpilou";
        let nonce: u64 = 1;
        let signature = "sigedpilou";
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
        assert_eq!(msg.public_key, public_key);
        assert_eq!(msg.nonce, nonce);
        assert_eq!(msg.signature, signature);
        assert_eq!(msg.payload, payload);
    }
}
