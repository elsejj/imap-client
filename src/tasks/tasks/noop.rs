use imap_next::imap_types::{
    command::CommandBody,
    response::{StatusBody, StatusKind},
};

use super::TaskError;
use crate::tasks::Task;

#[derive(Clone, Debug, Default)]
pub struct NoOpTask;

impl NoOpTask {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Task for NoOpTask {
    type Output = Result<(), TaskError>;

    fn command_body(&self) -> CommandBody<'static> {
        CommandBody::Noop
    }

    fn process_tagged(self, status_body: StatusBody<'static>) -> Self::Output {
        match status_body.kind {
            StatusKind::Ok => Ok(()),
            StatusKind::No => Err(TaskError::UnexpectedNoResponse(status_body)),
            StatusKind::Bad => Err(TaskError::UnexpectedBadResponse(status_body)),
        }
    }
}
