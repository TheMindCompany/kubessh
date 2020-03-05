# KubeSSH

A wrapper for `kubectl exec -it <pod> <container> /bin/bash` command.  Makes life a little simpler with user prompt for a pod list selection.

## Install

Use the below command to install binary or build from source.

Binary install:  

```bash
curl https://themindcompany.github.io/kubessh/install.sh -sS | bash -s
```

Source install:

```bash
git clone https://github.com/TheMindCompany/kubessh.git
cd kubessh
make build
make install
```

## USAGE

Refer to the help menu for details `-h` or `--help`.

```bash
USAGE:
    kubessh [FLAGS] [OPTIONS] [ARGS] [SUBCOMMAND]

FLAGS:
    -v, --verbose    Enable verbose logging
        --dry-run    Perform dry-run analysis
    -h, --help       Prints help information

OPTIONS:
    -f, --filter <filter>          Filter container list. ( ie: my-deployment-name )
    -n, --namespace <namespace>    Namespace target. ( ie: Environment )
    -c, --context <context>        Cluster target
        --eks <eks>                Update token for eks using aws profile

ARGS:
    <pod>          Pod target
    <container>    Container target

SUBCOMMANDS:
    configuration    Completion scripts for various shells
    help             Prints this message or the help of the given subcommand(s)
```


---

## Work In Progress
Feel free to contribute or use in any way.

Built with Rust love <3
