use crate::error::Result;

use super::{
    registration::{ConfirmRegistration, Register},
    sealedsender::{DecryptSealedMessageResponse, SealedSenderMessage},
};
use libsignal_service::provisioning::{VerifyAccountResponse, VerificationCodeResponse};
use tokio::sync::oneshot;

pub type Callback<T> = oneshot::Sender<Result<T>>;

pub enum Request {
    Register(Register, Callback<VerificationCodeResponse>),
    ConfirmRegistration(ConfirmRegistration, Callback<VerifyAccountResponse>),
    DecryptSealedSender(SealedSenderMessage, Callback<DecryptSealedMessageResponse>),
}
