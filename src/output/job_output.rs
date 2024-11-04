use serde::{Serialize, Deserialize};
use crate::job::job::Job;
use crate::workflow::stepflow::StepFlow;
use crate::workflow::taskflow::TaskFlow;

// This type is dedicated to being displayed as JSON output of a Job.
#[derive(Serialize, Deserialize)]
pub struct JobOutput {
    host: String,
    timestamp_start: String,
    timestamp_end: String,
    status: String,
    tasks: Vec<TaskOutput>
}

impl JobOutput {
    pub fn new() -> JobOutput {
        JobOutput {
            host: String::new(),
            timestamp_start: String::new(),
            timestamp_end: String::new(),
            status: String::new(),
            tasks: Vec::new()
        }
    }

    pub fn from_job(job: &Job) -> JobOutput {
        let mut job_output = JobOutput::new();

        job_output.host = job.get_address().unwrap();
        job_output.timestamp_start = job.timestamp_start.as_ref().unwrap().to_string();
        job_output.timestamp_end =job.timestamp_end.as_ref().unwrap().to_string();
        job_output.status = format!("{:?}", job.final_status);

        let mut tasks_output: Vec<TaskOutput> = Vec::new();
        for task_flow in job.hostworkflow.as_ref().unwrap().clone().task_flows {
            tasks_output.push(
                TaskOutput::from_taskflow(&task_flow)
            );
        }
        job_output.tasks = tasks_output;
        
        job_output
    }
}

#[derive(Serialize, Deserialize)]
pub struct TaskOutput {
    name: String,
    steps: Vec<StepOutput>
}

impl TaskOutput {
    pub fn from_taskflow(task_flow: &TaskFlow) -> TaskOutput {

        let mut steps_output: Vec<StepOutput> = Vec::new();
        for step_flow in task_flow.step_flows.clone() {
            steps_output.push(
                StepOutput::from_stepflow(&step_flow)
            );
        }

        TaskOutput {
            name: task_flow.name.as_ref().unwrap().to_string(),
            steps: steps_output
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct StepOutput {
    name: String,
    expected_state: String,
    status: String
}

impl StepOutput {
    pub fn from_stepflow(step_flow: &StepFlow) -> StepOutput {
        StepOutput {
            name: step_flow.step_expected.name.as_ref().unwrap().to_string(),
            expected_state: format!("{:?}", step_flow.step_expected.moduleblock),
            status: format!("{:?}", step_flow.step_status)
        }
    }
}
