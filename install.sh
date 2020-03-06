#!/bin/bash
VERSION=0.1.3

if [[ "$OSTYPE" == "linux-gnu" ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/TheMindCompany/kubessh/releases/download/${VERSION}/debian
  echo "Installing KubeSSH ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/kubessh
elif [[ "$OSTYPE" == "cygwin" ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/TheMindCompany/kubessh/releases/download/${VERSION}/debian
  echo "Installing KubeSSH ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/kubessh
elif [[ "$OSTYPE" == "debian"* ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/TheMindCompany/kubessh/releases/download/${VERSION}/debian
  echo "Installing KubeSSH ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/kubessh
elif [[ "$OSTYPE" == "msys" ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/TheMindCompany/kubessh/releases/download/${VERSION}/debian
  echo "Installing KubeSSH ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/kubessh
elif [[ "$OSTYPE" == "freebsd"* ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/TheMindCompany/kubessh/releases/download/${VERSION}/debian
  echo "Installing KubeSSH ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/kubessh
elif [[ "$OSTYPE" == "darwin"* ]]; then
  echo "Downloading darwin client."
  curl -LO https://github.com/TheMindCompany/kubessh/releases/download/${VERSION}/darwin
  echo "Installing KubeSSH ${VERSION}"
  chmod +x darwin
  mv darwin /usr/local/bin/kubessh
else
  printf "System not supported. Attempting to build from source. You must have rust installed."
  curl -LO https://github.com/TheMindCompany/kubessh/archive/${VERSION}.tar.gz
  tar -xvzf ${VERSION}.tar.gz
  cd ${VERSION}
  cargo build --release
  chmod +x target/release/kubessh
  mv target/release/kubessh /usr/local/bin/kubessh
  cd ..
  rm -rf kubessh-${VERSION}
fi

exit 0
