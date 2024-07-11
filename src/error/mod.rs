// Definition of all possible errors for the whole crate
// FIXME : relevant to subdivide this into multiple enums ?

#[derive(Debug)]
pub enum Error {
    FailureToFindGroupContent,
    FailureToParseContent(String),
    FailureToRunCommand(String),
    FailedInitialization(String),
    FailedTcpBinding(String),
    FailedTaskDryRun(String),
    MissingInitialization,
    GroupNotFound,
    MissingGroupsList,
    WrongInitialization,
}
