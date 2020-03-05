pub struct CompletionScript { }

    impl CompletionScript {
        pub fn bash() {
            println!("{}",r#"
    _kubessh() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            kubessh)
                cmd="kubessh"
                ;;
            
            bash)
                cmd+="__bash"
                ;;
            completions)
                cmd+="__completions"
                ;;
            configuration)
                cmd+="__configuration"
                ;;
            elvish)
                cmd+="__elvish"
                ;;
            fish)
                cmd+="__fish"
                ;;
            help)
                cmd+="__help"
                ;;
            install)
                cmd+="__install"
                ;;
            powershell)
                cmd+="__powershell"
                ;;
            zsh)
                cmd+="__zsh"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        kubessh)
            opts=" -v -h -f -n -c  --verbose --dry-run --help --filter --namespace --context --eks  <pod> <container>  configuration help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --filter)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -f)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --namespace)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -n)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --context)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --eks)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        
        kubessh__configuration)
            opts=" -h -V  --help --version   install completions help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        kubessh__configuration__completions)
            opts=" -h -V  --help --version   bash fish zsh powershell elvish help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        kubessh__configuration__completions__bash)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        kubessh__configuration__completions__elvish)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        kubessh__configuration__completions__fish)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        kubessh__configuration__completions__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        kubessh__configuration__completions__powershell)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        kubessh__configuration__completions__zsh)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        kubessh__configuration__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        kubessh__configuration__install)
            opts=" -h -V  --help --version  <packages>... "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        kubessh__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

complete -F _kubessh -o bashdefault -o default kubessh

    "#);
        }

        pub fn fish() {
            println!("{}",r#"
    complete -c kubessh -n "__fish_use_subcommand" -s f -l filter -d 'Filter container list. ( ie: my-deployment-name )'
complete -c kubessh -n "__fish_use_subcommand" -s n -l namespace -d 'Namespace target. ( ie: Environment )'
complete -c kubessh -n "__fish_use_subcommand" -s c -l context -d 'Cluster target'
complete -c kubessh -n "__fish_use_subcommand" -l eks -d 'Update token for eks using aws profile'
complete -c kubessh -n "__fish_use_subcommand" -s v -l verbose -d 'Enable verbose logging'
complete -c kubessh -n "__fish_use_subcommand" -l dry-run -d 'Perform dry-run analysis'
complete -c kubessh -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c kubessh -n "__fish_use_subcommand" -f -a "configuration" -d 'Completion scripts for various shells'
complete -c kubessh -n "__fish_use_subcommand" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c kubessh -n "__fish_seen_subcommand_from configuration" -s h -l help -d 'Prints help information'
complete -c kubessh -n "__fish_seen_subcommand_from configuration" -s V -l version -d 'Prints version information'
complete -c kubessh -n "__fish_seen_subcommand_from configuration" -f -a "install" -d 'Install packages needed to run this cli wrapper'
complete -c kubessh -n "__fish_seen_subcommand_from configuration" -f -a "completions" -d 'Completion scripts for various shells'
complete -c kubessh -n "__fish_seen_subcommand_from configuration" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c kubessh -n "__fish_seen_subcommand_from install" -s h -l help -d 'Prints help information'
complete -c kubessh -n "__fish_seen_subcommand_from install" -s V -l version -d 'Prints version information'
complete -c kubessh -n "__fish_seen_subcommand_from completions" -s h -l help -d 'Prints help information'
complete -c kubessh -n "__fish_seen_subcommand_from completions" -s V -l version -d 'Prints version information'
complete -c kubessh -n "__fish_seen_subcommand_from completions" -f -a "bash" -d 'Bash completion script'
complete -c kubessh -n "__fish_seen_subcommand_from completions" -f -a "fish" -d 'Fish completion script'
complete -c kubessh -n "__fish_seen_subcommand_from completions" -f -a "zsh" -d 'Zsh completion script'
complete -c kubessh -n "__fish_seen_subcommand_from completions" -f -a "powershell" -d 'PowerShell completion script'
complete -c kubessh -n "__fish_seen_subcommand_from completions" -f -a "elvish" -d 'Elvish completion script'
complete -c kubessh -n "__fish_seen_subcommand_from completions" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c kubessh -n "__fish_seen_subcommand_from bash" -s h -l help -d 'Prints help information'
complete -c kubessh -n "__fish_seen_subcommand_from bash" -s V -l version -d 'Prints version information'
complete -c kubessh -n "__fish_seen_subcommand_from fish" -s h -l help -d 'Prints help information'
complete -c kubessh -n "__fish_seen_subcommand_from fish" -s V -l version -d 'Prints version information'
complete -c kubessh -n "__fish_seen_subcommand_from zsh" -s h -l help -d 'Prints help information'
complete -c kubessh -n "__fish_seen_subcommand_from zsh" -s V -l version -d 'Prints version information'
complete -c kubessh -n "__fish_seen_subcommand_from powershell" -s h -l help -d 'Prints help information'
complete -c kubessh -n "__fish_seen_subcommand_from powershell" -s V -l version -d 'Prints version information'
complete -c kubessh -n "__fish_seen_subcommand_from elvish" -s h -l help -d 'Prints help information'
complete -c kubessh -n "__fish_seen_subcommand_from elvish" -s V -l version -d 'Prints version information'
complete -c kubessh -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c kubessh -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'
complete -c kubessh -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c kubessh -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'
complete -c kubessh -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c kubessh -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'

    "#);
        }

        pub fn zsh() {
            println!("{}",r#"
    #compdef kubessh

autoload -U is-at-least

_kubessh() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-f+[Filter container list. ( ie: my-deployment-name )]' \
'--filter=[Filter container list. ( ie: my-deployment-name )]' \
'-n+[Namespace target. ( ie: Environment )]' \
'--namespace=[Namespace target. ( ie: Environment )]' \
'-c+[Cluster target]' \
'--context=[Cluster target]' \
'--eks=[Update token for eks using aws profile]' \
'-v[Enable verbose logging]' \
'--verbose[Enable verbose logging]' \
'--dry-run[Perform dry-run analysis]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'::pod -- Pod target:_files' \
'::container -- Container target:_files' \
":: :_kubessh_commands" \
"*::: :->kubessh" \
&& ret=0
    case $state in
    (kubessh)
        words=($line[3] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:kubessh-command-$line[3]:"
        case $line[3] in
            (configuration)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_kubessh__configuration_commands" \
"*::: :->configuration" \
&& ret=0
case $state in
    (configuration)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:kubessh-configuration-command-$line[1]:"
        case $line[1] in
            (install)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::packages -- Available installation packages:_files' \
&& ret=0
;;
(completions)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_kubessh__configuration__completions_commands" \
"*::: :->completions" \
&& ret=0
case $state in
    (completions)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:kubessh-configuration-completions-command-$line[1]:"
        case $line[1] in
            (bash)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Bash completion script:_files' \
&& ret=0
;;
(fish)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Fish completion script:_files' \
&& ret=0
;;
(zsh)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Zsh completion script:_files' \
&& ret=0
;;
(powershell)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- PowerShell completion script:_files' \
&& ret=0
;;
(elvish)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Elvish completion script:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
}

(( $+functions[_kubessh_commands] )) ||
_kubessh_commands() {
    local commands; commands=(
        "configuration:Completion scripts for various shells" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'kubessh commands' commands "$@"
}
(( $+functions[_kubessh__configuration__completions__bash_commands] )) ||
_kubessh__configuration__completions__bash_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'kubessh configuration completions bash commands' commands "$@"
}
(( $+functions[_kubessh__configuration__completions_commands] )) ||
_kubessh__configuration__completions_commands() {
    local commands; commands=(
        "bash:Bash completion script" \
"fish:Fish completion script" \
"zsh:Zsh completion script" \
"powershell:PowerShell completion script" \
"elvish:Elvish completion script" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'kubessh configuration completions commands' commands "$@"
}
(( $+functions[_kubessh__configuration_commands] )) ||
_kubessh__configuration_commands() {
    local commands; commands=(
        "install:Install packages needed to run this cli wrapper" \
"completions:Completion scripts for various shells" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'kubessh configuration commands' commands "$@"
}
(( $+functions[_kubessh__configuration__completions__elvish_commands] )) ||
_kubessh__configuration__completions__elvish_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'kubessh configuration completions elvish commands' commands "$@"
}
(( $+functions[_kubessh__configuration__completions__fish_commands] )) ||
_kubessh__configuration__completions__fish_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'kubessh configuration completions fish commands' commands "$@"
}
(( $+functions[_kubessh__configuration__completions__help_commands] )) ||
_kubessh__configuration__completions__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'kubessh configuration completions help commands' commands "$@"
}
(( $+functions[_kubessh__configuration__help_commands] )) ||
_kubessh__configuration__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'kubessh configuration help commands' commands "$@"
}
(( $+functions[_kubessh__help_commands] )) ||
_kubessh__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'kubessh help commands' commands "$@"
}
(( $+functions[_kubessh__configuration__install_commands] )) ||
_kubessh__configuration__install_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'kubessh configuration install commands' commands "$@"
}
(( $+functions[_kubessh__configuration__completions__powershell_commands] )) ||
_kubessh__configuration__completions__powershell_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'kubessh configuration completions powershell commands' commands "$@"
}
(( $+functions[_kubessh__configuration__completions__zsh_commands] )) ||
_kubessh__configuration__completions__zsh_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'kubessh configuration completions zsh commands' commands "$@"
}

_kubessh "$@"
    "#);
        }

        pub fn powershell() {
            println!("{}",r#"
    
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'kubessh' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'kubessh'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'kubessh' {
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'Filter container list. ( ie: my-deployment-name )')
            [CompletionResult]::new('--filter', 'filter', [CompletionResultType]::ParameterName, 'Filter container list. ( ie: my-deployment-name )')
            [CompletionResult]::new('-n', 'n', [CompletionResultType]::ParameterName, 'Namespace target. ( ie: Environment )')
            [CompletionResult]::new('--namespace', 'namespace', [CompletionResultType]::ParameterName, 'Namespace target. ( ie: Environment )')
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'Cluster target')
            [CompletionResult]::new('--context', 'context', [CompletionResultType]::ParameterName, 'Cluster target')
            [CompletionResult]::new('--eks', 'eks', [CompletionResultType]::ParameterName, 'Update token for eks using aws profile')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Enable verbose logging')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Enable verbose logging')
            [CompletionResult]::new('--dry-run', 'dry-run', [CompletionResultType]::ParameterName, 'Perform dry-run analysis')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('configuration', 'configuration', [CompletionResultType]::ParameterValue, 'Completion scripts for various shells')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'kubessh;configuration' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('install', 'install', [CompletionResultType]::ParameterValue, 'Install packages needed to run this cli wrapper')
            [CompletionResult]::new('completions', 'completions', [CompletionResultType]::ParameterValue, 'Completion scripts for various shells')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'kubessh;configuration;install' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'kubessh;configuration;completions' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('bash', 'bash', [CompletionResultType]::ParameterValue, 'Bash completion script')
            [CompletionResult]::new('fish', 'fish', [CompletionResultType]::ParameterValue, 'Fish completion script')
            [CompletionResult]::new('zsh', 'zsh', [CompletionResultType]::ParameterValue, 'Zsh completion script')
            [CompletionResult]::new('powershell', 'powershell', [CompletionResultType]::ParameterValue, 'PowerShell completion script')
            [CompletionResult]::new('elvish', 'elvish', [CompletionResultType]::ParameterValue, 'Elvish completion script')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'kubessh;configuration;completions;bash' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'kubessh;configuration;completions;fish' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'kubessh;configuration;completions;zsh' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'kubessh;configuration;completions;powershell' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'kubessh;configuration;completions;elvish' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'kubessh;configuration;completions;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'kubessh;configuration;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'kubessh;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}

    "#);
        }

        pub fn elvish() {
            println!("{}",r#"
            
edit:completion:arg-completer[kubessh] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'kubessh'
    for word $words[1:-1] {
        if (has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'kubessh'= {
            cand -f 'Filter container list. ( ie: my-deployment-name )'
            cand --filter 'Filter container list. ( ie: my-deployment-name )'
            cand -n 'Namespace target. ( ie: Environment )'
            cand --namespace 'Namespace target. ( ie: Environment )'
            cand -c 'Cluster target'
            cand --context 'Cluster target'
            cand --eks 'Update token for eks using aws profile'
            cand -v 'Enable verbose logging'
            cand --verbose 'Enable verbose logging'
            cand --dry-run 'Perform dry-run analysis'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand configuration 'Completion scripts for various shells'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'kubessh;configuration'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand install 'Install packages needed to run this cli wrapper'
            cand completions 'Completion scripts for various shells'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'kubessh;configuration;install'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'kubessh;configuration;completions'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand bash 'Bash completion script'
            cand fish 'Fish completion script'
            cand zsh 'Zsh completion script'
            cand powershell 'PowerShell completion script'
            cand elvish 'Elvish completion script'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'kubessh;configuration;completions;bash'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'kubessh;configuration;completions;fish'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'kubessh;configuration;completions;zsh'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'kubessh;configuration;completions;powershell'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'kubessh;configuration;completions;elvish'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'kubessh;configuration;completions;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'kubessh;configuration;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'kubessh;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}

    "#);
        }
    }
    