// Definition of all possible errors for the whole crate
// FIXME : relevant to subdivide this into multiple enums ?

#[derive(Debug)]
pub enum Error {
    FailureToFindGroupContent,
    FailureToParseContent(String),
    FailureToRunCommand(String),
    FailureToEstablishConnection(String),
    FailedInitialization(String),
    FailedTcpBinding(String),
    FailedTaskDryRun(String),
    MissingInitialization(String),
    GroupNotFound,
    MissingGroupsList,
    WrongInitialization,
}
