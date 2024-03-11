use quote::ToTokens;
use syn::Error;

pub(crate) enum ErrorKind {
    SpanLater(String),
    Spanned(Error),
}

pub(crate) type Result<T> = std::result::Result<T, ErrorKind>;

impl From<Error> for ErrorKind {
    fn from(value: Error) -> Self {
        Self::Spanned(value)
    }
}

impl<T> Into<Result<T>> for ErrorKind {
    fn into(self) -> Result<T> {
        Err(self)
    }
}

impl ErrorKind {
    pub(crate) fn span_now<T: ToTokens>(self, token: T) -> Self {
        match self {
            Self::SpanLater(msg) => Self::Spanned(Error::new_spanned(token, msg)),
            spanned => spanned,
        }
    }
}
