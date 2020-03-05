use crate::command_control::cmd_model::cmdctl::CmdCtl;
use crate::prompt_user::handler::*;
use crate::prompt_user::model::*;
use crate::toolbelt::*;

#[derive(Debug, Default, Clone)]
pub struct KubeObjectListPrompt {
    dry_run: bool,
    verbose: bool,
    eks: String,
    filter: String,
    context: String,
    namespace: String,
}

impl KubeObjectListPrompt {
    pub fn new() -> KubeObjectListPrompt {
        Default::default()
    }

    pub fn process_object_resources(&mut self, name: String, mut options: Vec<String>) -> Option<String> {
        let question = format!("Please choose a {}", name);
        options.push("exit".to_string());
        let prompt = PromptOptions::new().set_name(&question).set_options(&options).set_default(&"exit".to_string());

        match PromptOptionsHandler::process(&prompt) {
            Some(response) => {
                if response == "exit" {
                    None
                }
                else {
                    Some(response)
                }
            },
            None => None,
        }
    }

    pub fn process(&mut self, mut request: CmdCtl) -> CmdCtl {
        self.dry_run = request.dry_run;
        self.verbose = request.verbose;

        match request.context {
            Some(context) => {
                request.context = Some(context.clone());
                self.context = context;
            },
            None => {
                let current_context = self.get_context();
                request.context = Some(current_context.clone());
                self.context = current_context;
            }
        }

        if let Some(eks) = request.eks {
            self.eks = eks.clone();
            request.eks = Some(eks);
            self.update_eks_token();
        }

        if let Some(namespace) = request.namespace {
            self.namespace = namespace.clone();
            request.namespace = Some(namespace);
        }

        if let Some(filter) = request.filter {
            self.filter = filter.clone();
            request.filter = Some(filter);
        }

        match request.pod {
            Some(pod) => {
                request.pod =  Some(pod);
            },
            None => {
                let pods = self.pod_list();
                request.pod = self.process_object_resources(
                    "pod".to_string(), pods
                );
            }
        }

        request.clone()
    }

    pub fn get_context(&self) -> String {
        let mut handler = CommandLineHandler::new();
        handler.set_exit_on_error(true);
        handler.run_cmd(
            "kubectl config current-context",
            self.dry_run,
            self.verbose
        )
    }

    pub fn pod_list(&self) -> Vec<String> {
        let mut cmd = "kubectl ".to_string();
        let mut handler = CommandLineHandler::new();
        handler.set_exit_on_error(true);

        if !self.context.is_empty() {
            cmd.push_str("--context='");
            cmd.push_str(&self.context);
            cmd.push_str("' ");
        }

        if !self.namespace.is_empty() {
            cmd.push_str("--namespace='");
            cmd.push_str(&self.namespace);
            cmd.push_str("' ");
        }
        cmd.push_str("get po --no-headers -o custom-columns=Name:.metadata.name ");

        if !self.filter.is_empty() {
            cmd.push_str("| grep ");
            cmd.push_str(&self.filter);
            cmd.push_str(" ");
        }

        let mut my_vals: Vec<String> = handler
            .run_cmd(&cmd, self.dry_run, self.verbose)
            .split('\n')
            .map(| a | a.to_string())
            .collect();

        my_vals.remove(my_vals.len() -1);
        my_vals
    }

    pub fn update_eks_token(&self) {
        let mut handler = CommandLineHandler::new();
        handler.set_exit_on_error(true);

        let mut cmd = "aws --profile ".to_string();
        cmd.push_str(&self.eks);
        cmd.push_str(" eks update-kubeconfig --name ");
        cmd.push_str(&self.context);
        cmd.push_str(" --alias ");
        cmd.push_str(&self.context);

        handler.run_cmd(&cmd, self.dry_run, self.verbose);
    }
}
