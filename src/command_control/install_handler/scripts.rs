
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

echo "Script: kubectl"
"#.to_string()
    }

    
    pub fn linux_awstools() -> String {
        r#"#!/bin/bash

echo "Script: aws"

echo "This script has not been created.  Please help contribute by creating a script for your OS that others can use and commit to config-manager."
"#.to_string()
    }

    
    pub fn osx_awstools() -> String {
        r#"#!/bin/bash

echo "Script: aws"

echo "This script has not been created.  Please help contribute by creating a script for your OS that others can use and commit to config-manager."
"#.to_string()
    }

    
    pub fn osx_kubectl() -> String {
        r#"#!/bin/bash

echo "Script: kubectl"
"#.to_string()
    }

}