pub struct ModTemplate {

}

impl ModTemplate {
    pub fn get_top_template() -> String {
        String::from(
            r#"pub struct CompletionScript { }

    impl CompletionScript {
        pub fn bash() {
            println!("{}",r#"
    "#,
        )
    }

    pub fn get_fish_template() -> String {
        String::from(
            "
    \"#);
        }

        pub fn fish() {
            println!(\"{}\",r#\"
    ",
        )
    }

    pub fn get_zsh_template() -> String {
        String::from(
            "
    \"#);
        }

        pub fn zsh() {
            println!(\"{}\",r#\"
    ",
        )
    }

    pub fn get_ps1_template() -> String {
        String::from(
            "
    \"#);
        }

        pub fn powershell() {
            println!(\"{}\",r#\"
    ",
        )
    }

    pub fn get_elvish_template() -> String {
        String::from(
            "
    \"#);
        }

        pub fn elvish() {
            println!(\"{}\",r#\"
            ",
        )
    }

    pub fn get_bottom_template() -> String {
        String::from(
            "
    \"#);
        }
    }
    ",
        )
    }
}
