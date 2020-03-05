use std::fs::File;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use handlebars::{Handlebars, TemplateRenderError};
use super::install_build_filepath::InstallScriptPath;
use serde_json;
use serde_json::value::{Map};
use serde_json::value::Value as Json;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct Function {
    pub name: String,
    pub script: String,
    pub function: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InstallScriptModBuilder {}

impl InstallScriptModBuilder {

    pub fn new() -> InstallScriptModBuilder {
        Default::default()
    }

    pub fn create(&self) {
        let file_path = Path::new("./src/command_control/install_handler/scripts.rs");
        let scripts = InstallScriptPath::new().collect_script_paths();
        let mut functions = Vec::new();

        for script in scripts {
            let tmp = self.create_function(script);
            functions.push(tmp);
        }

        File::create(&file_path).unwrap();
        self.create_mod(&functions, &file_path.to_path_buf());
    }

    fn create_mod(&self, params: &Vec<Json>, script_path: &PathBuf) {
        let mut handler = Handlebars::new();
        handler.register_escape_fn(handlebars::no_escape);
        let template = r#"
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
            {{#each params}}
            &"{{name}}" => InstallScript::{{name}}(),
            {{/each}}
            _ => "".to_string(),
        }
    }
{{#each params}}
    {{this.function}}
{{/each}}
}"#;
        let mut map = Map::new();
        map.insert("params".to_string(), Json::Array(params.clone()));

        let module = handler.render_template(template, &map);
        match module {
            Ok(x) => {
                println!("Created script module.\n{:#?}", x);
                self.file_append(script_path, &x);
            },
            Err(err) => {
                println!("render module error: \n{:#?}", err);
                std::process::exit(1);
            }
        }
    }

    fn create_function(&self, script_path: PathBuf) -> Json {
        let mut handler = Handlebars::new();
        handler.register_escape_fn(handlebars::no_escape);
        let filename = script_path.file_stem().unwrap().to_os_string().to_str().unwrap().to_string();
        let template = "
    pub fn {{name}}() -> String {
        r#\"{{script}}\"#.to_string()
    }";
        let mut script = Function {
            name: filename.clone(),
            script: self.read_into_string(&script_path),
            function: "".to_string(),
        };

        script.function = self.unwrap_function(handler.render_template(template, &script));

        match serde_json::to_value(script) {
            Ok(x) => {
                x
            },
            Err(err) => {
                println!("render function error: \n{:#?}", err);
                std::process::exit(1);
            }
        }
    }

    fn unwrap_function(&self, function: Result<String, TemplateRenderError >) -> String {
        match function {
            Ok(x) => {
                x
            },
            Err(err) => {
                println!("render function error: \n{:#?}", err);
                std::process::exit(1);
            }
        }
    }

    pub fn read_into_string(&self, file: &PathBuf) -> String {
        let f = fs::read_to_string(file.clone());
        match f {
            Ok(x) => {
                println!("Opened: \n{:#?}", x);
                x
            },
            Err(err) => {
                println!("read file error: \n{:#?}", err);
                std::process::exit(1);
            }
        }
    }
    fn file_append(&self, out: &PathBuf, template: &String) {
        let mut outfile = OpenOptions::new()
            .write(true)
            .append(true)
            .open(out)
            .unwrap();
        outfile
            .write_fmt(format_args!("{}", template))
            .expect("Error writing updated scripts.rs module.");
    }

}
