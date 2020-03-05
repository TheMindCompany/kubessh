# Gimbal Config Manager

Tool for navigating and using software architecture services configuration for Gimbal Inc.

**Current supported configuration services:**
- k8s ( https://github.com/PaeDae/k8s )
- ansible ( https://github.com/PaeDae/ansible)

**Feature Highlights**
- Built for engineering and development at Gimbal to manage service architecture
- Completion script for all major shells
- Self configuration on first run
- Prompt navigation for target content
- Managed source content for itself (ie: repository/branch)
- Creates AWS config/credential file if not found
- Built with mac & linux in mind but should work on windows
- Install commonly used packages for engineering and development efforts at Gimbal

## Cli

Navigating `config-manager` can be done with the `--help` flag to obtain information on each command/subcommand and usage.  

### Configuration

In an effort to make configuration of `config-manager` simple we have defined a single YAML config that will maintain configurations.  All external sources are synced to the users `~/.configmanager` and managed through `~/.configmanager/config.yaml`.  

The first time you run the cli utility you will be prompted to configure if not present.  

**~/.configmanager/config.yaml**  
```yaml
kind: config
version: alpha/1.0
specs:
  credentials:
    user_full: Joe Strummer
    user_email: joe@mescandetos.com
    aws:
    - name: majority
      key: ***
      secret: ***
    - name: gimbal
      key: ***
      secret: ***
  services:
    k8s:
      repo: PaeDae/k8s
      branch: master
    ansible:
      repo: PaeDae/ansible
      branch: master
```

### Autocompletion

For convenience purposes autocompletion scripts have been provided for most major shell programs.

More information for each completion script provided:
```bash
config-manager system-config completions --help
```


### Install Engineering & Development Packages

In order to speed up the onboarding process there is a feature that detects your platform and installs commonly used packages used for engineering and development at Gimbal.  These installations are cli based scripts found in `src/comand_control/install_handler/generator/scripts` path and are generated into code at build time for the compiler.

We are still working to create install scripts for each of these packages; many are there.  If a package is not available it will notify you and request you to help contribute in creating one.

Install all packages:
```bash
config-manager system-config install
```

### k8s Configuration Repository

Run k8s deployments from your desired k8s repository branch.  Use prompt navigation to target your deployment when the name is unknown.

**Examples**  

Use prompt navigation:  
```bash
config-manager k8s -p
```

Do a dry run with recent repository updates:  
```bash
config-manager k8s eks-busbar-cluster busbar --dry-run
```

### Ansible Configuration Repository

Run Ansible based processes from ansible repository branch.  Use prompt navigation to target assist you in finding the unknown.

**Examples**  

Use prompt navigation on playbook:  
```bash
config-manager ansible playbook -p
```

Do a dry run playbook with recent repository updates:  
```bash
config-manager ansible playbook majority provision-redis-cluster.yml --dry-run
```

Use prompt navigation on playbook task:  
```bash
config-manager ansible task -p
```

Use prompt navigation on ami build:  
```bash
config-manager ansible build -p
```

---

## Work In Progress
Feel free to contribute or use in any way.

Built with Rust love <3
