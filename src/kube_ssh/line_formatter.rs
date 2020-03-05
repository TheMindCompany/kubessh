use crate::command_control::cmd_model::cmdctl::CmdCtl;

pub fn format_request(request: &CmdCtl) -> String {
    let mut command = "kubectl ".to_string();

    match &request.context {
        Some(context) => {
            command.push_str("--context='");
            command.push_str(&context);
            command.push_str("' ");
        },
        None => {
            // Do nothing assume context is set.
            // If this happens though, how???
        }
    }

    match &request.namespace {
        Some(namespace) => {
            command.push_str("--namespace='");
            command.push_str(&namespace);
            command.push_str("' ");
        },
        None => {
            // Do nothing assume default namespace.
        }
    }

    command.push_str("exec -it ");

    match &request.pod {
        Some(pod) => {
            command.push_str(&pod);
            command.push_str(" ");
        },
        None => {
            // Umm.  It is time to exit this joint!
            std::process::exit(1);
        }
    }

    match &request.container {
        Some(container) => {
            command.push_str("--container='");
            command.push_str(&container);
            command.push_str("' ");
        },
        None => {
            // Do nothing, let kubectl decide.
        }
    }

    command.push_str("/bin/bash ");

    command
}
