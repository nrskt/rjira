use std::error::Error as StdError;

use eyre::{Report, WrapErr};

pub trait WrapErrExt<T, E>: WrapErr<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    #[track_caller]
    fn wrap<ErrType>(self) -> Result<T, Report>
    where
        E: Into<ErrType>,
        ErrType: StdError + Send + Sync + 'static;

    #[track_caller]
    fn wrap_msg<ErrType>(self, msg: &str) -> Result<T, Report>
    where
        E: Into<ErrType>,
        ErrType: StdError + Send + Sync + 'static;
}

impl<T, E> WrapErrExt<T, E> for Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    fn wrap<ErrType>(self) -> Result<T, Report>
    where
        E: Into<ErrType>,
        ErrType: StdError + Send + Sync + 'static,
    {
        match self {
            Ok(r) => Ok(r),
            Err(e) => Err(Report::new(e.into())),
        }
    }

    fn wrap_msg<ErrType>(self, msg: &str) -> Result<T, Report>
    where
        E: Into<ErrType>,
        ErrType: StdError + Send + Sync + 'static,
    {
        match self {
            Ok(r) => Ok(r),
            Err(e) => {
                let report = Report::new(e);
                let report = report.wrap_err(format!("{}", msg));
                Err(report)
            }
        }
    }
}
