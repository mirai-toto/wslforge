use inquire::InquireError;

#[derive(Clone, Copy, PartialEq)]
pub enum Step {
    Hostname,
    Username,
    Password,
    Override,
    Proxy,
    CloudInit,
    Image,
}

impl Step {
    pub fn prev(self) -> Self {
        match self {
            Self::Hostname => Self::Hostname,
            Self::Username => Self::Hostname,
            Self::Password => Self::Username,
            Self::Override => Self::Password,
            Self::Proxy => Self::Override,
            Self::CloudInit => Self::Proxy,
            Self::Image => Self::CloudInit,
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Hostname => Self::Username,
            Self::Username => Self::Password,
            Self::Password => Self::Override,
            Self::Override => Self::Proxy,
            Self::Proxy => Self::CloudInit,
            Self::CloudInit => Self::Image,
            Self::Image => Self::Image,
        }
    }
}

pub fn is_back(err: &InquireError) -> bool {
    matches!(err, InquireError::OperationCanceled)
}

pub fn is_anyhow_back(err: &anyhow::Error) -> bool {
    err.downcast_ref::<InquireError>().map(is_back).unwrap_or(false)
}
