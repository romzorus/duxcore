use crate::assignment::assignment::Assignment;
use crate::assignment::assignment::AssignmentFinalStatus;
use crate::prelude::HostWorkFlow;
use crate::result::apicallresult::ApiCallStatus;
use crate::task::step::Step;
use colored::Colorize;
use termimad::crossterm::style::Color::*;
use termimad::*;

// TODO : have this work with an &Assignment instead of an Assignment
pub fn display_output(assignment: Assignment) {
    println!(
        "Host {} : {}",
        assignment.host.bold(),
        output_nice_finalstatus(&assignment.finalstatus)
    );

    match assignment.finalstatus {
        AssignmentFinalStatus::Unset => {
            println!("{}", "Assignment is ready to be applied".bold());
        }
        AssignmentFinalStatus::AlreadyMatched => {
            // TODO : more details ?
        }
        AssignmentFinalStatus::FailedDryRun(error) => {
            println!("{}\n", error.red());
            // TODO : show where it failed exactly in the TaskList
        }
        AssignmentFinalStatus::Changed => {
            show_tasklistresult(assignment);
        }
        AssignmentFinalStatus::ChangedWithFailures => {
            show_tasklistresult(assignment);
        }
        AssignmentFinalStatus::FailedChange => {
            show_tasklistresult(assignment);
        }
        AssignmentFinalStatus::GenericFailed(error) => {
            println!("{}\n", error.red());
        }
    }
}

//pub fn display_results_detailed() {}

//pub fn display_results_summary() {}

fn output_nice_result(status: &ApiCallStatus) -> String {
    match status {
        ApiCallStatus::None => String::from("None"),
        ApiCallStatus::Unset => String::from("None"),
        ApiCallStatus::ChangeSuccessful(message) => {
            format!("Success : {}", message)
        }
        ApiCallStatus::Failure(message) => {
            format!("Failure : {}", message)
        }
        ApiCallStatus::AllowedFailure(message) => {
            format!("Failure (allowed): {}", message)
        }
    }
}

// TODO : improve this / replace with step name when it will be implemented
fn output_nice_step(step: &Step) -> String {
    match step.name.clone() {
        None => {
            return format!("`{:?}`", step);
        }
        Some(content) => {
            return content;
        }
    }
}

fn output_nice_finalstatus(finalstatus: &AssignmentFinalStatus) -> String {
    match finalstatus {
        AssignmentFinalStatus::Unset => {
            return format!("{}", "Unset".red().bold()); // Should never occur
        }
        AssignmentFinalStatus::FailedDryRun(_error) => {
            return format!("{}", "Failed dry run".red().bold());
        }
        AssignmentFinalStatus::Changed => {
            return format!("{}", "Changed".blue().bold());
        }
        AssignmentFinalStatus::ChangedWithFailures => {
            return format!("{}", "Changed (with failures)".truecolor(255, 90, 0).bold());
        }
        AssignmentFinalStatus::FailedChange => {
            return format!("{}", "Failed change".red().bold());
        }
        AssignmentFinalStatus::AlreadyMatched => {
            return format!("{}", "Matched".green().bold());
        }
        AssignmentFinalStatus::GenericFailed(error) => {
            return format!("{}", format!("Failed : {}", error).red().bold());
        }
    }
}

fn show_tasklistresult(assignment: Assignment) {
    let mut skin = MadSkin::default();
    skin.set_headers_fg(rgb(255, 187, 0));
    skin.bold.set_fg(White);

    // 1 display per Task
    for taskindex in 0..assignment.resultlist.taskresults.len() {
        match &assignment.resultlist.taskresults[taskindex].stepresults {
            None => {
                println!(
                    "Task : {} -> {}",
                    &assignment.tasklist.tasks[taskindex]
                        .name
                        .clone()
                        .unwrap_or(String::from("no name for TaskBlock"))
                        .bold(),
                    "no result".bold()
                );
            }
            Some(stepresults) => {
                println!(
                    "Task : {}",
                    &assignment.tasklist.tasks[taskindex]
                        .name
                        .clone()
                        .unwrap_or(String::from("no name for TaskBlock"))
                        .bold()
                );

                let mut table_content = String::new();
                table_content.push_str("|:-:|:-:|-");
                table_content.push_str("\n|**Step**|**Changes**|**Results**|");
                table_content.push_str("\n|-");

                for (stepindex, stepresultcontent) in stepresults.iter().enumerate() {
                    // One step can represent multiple changes so the 1st line is displayed by itself, with the name
                    // of the step, then the rest without this name

                    table_content.push_str(
                        format!(
                            "\n|{}|{}|{}|",
                            output_nice_step(
                                &assignment.tasklist.tasks[taskindex].steps[stepindex]
                            ),
                            assignment.changelist.taskchanges.clone().unwrap()[taskindex]
                                .stepchanges
                                .clone()[stepindex]
                                .display()[0],
                            output_nice_result(
                                &assignment.resultlist.clone().taskresults[taskindex]
                                    .stepresults
                                    .clone()
                                    .unwrap()[stepindex]
                                    .apicallresults[0]
                                    .status
                            )
                        )
                        .as_str(),
                    );

                    for (apicallindex, _apicallcontent) in
                        stepresultcontent.apicallresults.iter().enumerate()
                    {
                        if apicallindex > 0 {
                            table_content.push_str(
                                format!(
                                    "\n||{}|{}|",
                                    assignment.changelist.taskchanges.clone().unwrap()[taskindex]
                                        .stepchanges
                                        .clone()[stepindex]
                                        .display()[apicallindex],
                                    output_nice_result(
                                        &assignment.resultlist.clone().taskresults[taskindex]
                                            .stepresults
                                            .clone()
                                            .unwrap()[stepindex]
                                            .apicallresults[apicallindex]
                                            .status
                                    )
                                )
                                .as_str(),
                            );
                        }
                    }
                }

                // Close the table and display it
                table_content.push_str("\n|-");
                println!("{}", skin.term_text(&table_content));

                // If the last result is a Failure, display details about it
                match &stepresults
                    .last()
                    .unwrap()
                    .apicallresults
                    .last()
                    .unwrap()
                    .status
                {
                    ApiCallStatus::Failure(_) => {
                        println!(
                            "{}",
                            &stepresults
                                .last()
                                .unwrap()
                                .apicallresults
                                .last()
                                .unwrap()
                                .output
                                .as_ref()
                                .unwrap()
                                .red()
                        );
                    }
                    _ => {}
                }
            }
        }
    }
}
