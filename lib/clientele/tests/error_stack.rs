// This is free and unencumbered software released into the public domain.

//! Checks error-stack integration through Clientele's public re-exports.

#[cfg(feature = "std")]
use clientele::crates::error_stack::ResultExt;
use clientele::{crates::error_stack::Report, SysexitsError, SysexitsResult};

#[test]
fn sysexits_errors_can_be_report_contexts() {
    let result: SysexitsResult<()> = Err(SysexitsError::EX_USAGE);
    let report = result.map_err(Report::new).unwrap_err();

    assert_eq!(*report.current_context(), SysexitsError::EX_USAGE);
}

#[cfg(feature = "std")]
#[test]
fn changing_to_sysexits_context_preserves_the_source_error() {
    let error = std::io::Error::from(std::io::ErrorKind::NotFound);
    let context = SysexitsError::from(&error);
    let result: std::io::Result<()> = Err(error);
    let report = result.change_context(context).unwrap_err();

    assert_eq!(*report.current_context(), SysexitsError::EX_NOINPUT);
    assert_eq!(
        report.downcast_ref::<std::io::Error>().unwrap().kind(),
        std::io::ErrorKind::NotFound,
    );
}
