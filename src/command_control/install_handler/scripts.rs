
use sys_info;

#[derive(Debug, Default, Clone)]
pub struct InstallScript { }

impl InstallScript {

    pub fn get_script_named(script_name: String) -> String {
        let mut function_name = String::new();
        match sys_info::os_type() {
            Ok(os) => {
                match os.as_str() {
                    "Linux" => {
                        function_name.push_str("linux_");
                        function_name.push_str(&script_name);
                    },
                    "Darwin" => {
                        function_name.push_str("osx_");
                        function_name.push_str(&script_name);
                    },
                    "Windows" => {
                        function_name.push_str("ms_");
                        function_name.push_str(&script_name);
                    },
                    _ => {
                        function_name.push_str(&script_name);
                    }
                }
            },
            Err(_) => {
                function_name.push_str(&script_name);
            }
        }

        match &function_name.as_str() {
            
            &"linux_kubectl" => InstallScript::linux_kubectl(),
            
            &"linux_awstools" => InstallScript::linux_awstools(),
            
            &"osx_awstools" => InstallScript::osx_awstools(),
            
            &"osx_kubectl" => InstallScript::osx_kubectl(),
            
            _ => "".to_string(),
        }
    }

    
    pub fn linux_kubectl() -> String {
        r#"#!/bin/bash

curl -LO https://storage.googleapis.com/kubernetes-release/release/`curl -s https://storage.googleapis.com/kubernetes-release/release/stable.txt`/bin/linux/amd64/kubectl
"#.to_string()
    }

    
    pub fn linux_awstools() -> String {
        r#"#!/bin/bash

curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
unzip awscliv2.zip
sudo ./aws/install

curl -o aws-iam-authenticator https://amazon-eks.s3-us-west-2.amazonaws.com/1.14.6/2019-08-22/bin/linux/amd64/aws-iam-authenticator
chmod +x ./aws-iam-authenticator
mkdir -p $HOME/bin && cp ./aws-iam-authenticator $HOME/bin/aws-iam-authenticator && export PATH=$PATH:$HOME/bin

echo 'export PATH=$PATH:$HOME/bin' >> ~/.bashrc
echo 'export PATH=$PATH:$HOME/bin' >> ~/.bash_profile

aws-iam-authenticator help
"#.to_string()
    }

    
    pub fn osx_awstools() -> String {
        r#"#!/bin/bash

curl "https://awscli.amazonaws.com/AWSCLIV2.pkg" -o "AWSCLIV2.pkg"
sudo installer -pkg AWSCLIV2.pkg -target /

curl -o aws-iam-authenticator https://amazon-eks.s3-us-west-2.amazonaws.com/1.14.6/2019-08-22/bin/darwin/amd64/aws-iam-authenticator
chmod +x ./aws-iam-authenticator
mkdir -p $HOME/bin && cp ./aws-iam-authenticator $HOME/bin/aws-iam-authenticator && export PATH=$PATH:$HOME/bin

echo 'export PATH=$PATH:$HOME/bin' >> ~/.bashrc
echo 'export PATH=$PATH:$HOME/bin' >> ~/.bash_profile

aws-iam-authenticator help
"#.to_string()
    }

    
    pub fn osx_kubectl() -> String {
        r#"#!/bin/bash

curl -LO "https://storage.googleapis.com/kubernetes-release/release/$(curl -s https://storage.googleapis.com/kubernetes-release/release/stable.txt)/bin/darwin/amd64/kubectl"
"#.to_string()
    }

}