#!/usr/bin/env bash

VERSION=0.1
WORK_DIR=$(cd $(dirname $0); pwd)
AUTH_MODULE=authority_management
ERC20_MODULE=erc20
ERC20FACTORY_MODULE=erc20_factory
GOVNANCE_MODULE=govnance_dao
INCOME_MODULE=income_category
KERNEL_MODULE=kernel
MULTISIG_MODULE=multisig
MULTISIGFACTORY_MODULE=multisig_factory
ROLEMANAGE_MODULE=role_manage
ROUTE_MODULE=route_manage
USERS_MODULE=users_manage

function build_module() {
    m_name=$1
    m_dir=${WORK_DIR}/${m_name}
    echo "build module ${m_dir}"
    cd ${m_dir}
    cargo +nightly contract build
    if [ $? -ne 0 ];then
      echo "build module failed"
      exit 1
    fi
    echo "copy to ../release"
    cp ${m_dir}/target/ink/${m_name}.wasm ../release/${m_name}_v$VERSION.wasm
    cp ${m_dir}/target/ink/${m_name}.contract ../release/${m_name}_v$VERSION.contract
    cp ${m_dir}/target/ink/metadata.json ../release/${m_name}_v$VERSION.json
    cd -
}

echo "clean release"
rm -rf ${WORK_DIR}/release
mkdir -p ${WORK_DIR}/release

build_module ${AUTH_MODULE}
build_module ${ERC20FACTORY_MODULE}
build_module ${ERC20_MODULE}
# build_module ${GITHUB_MODULE}
build_module ${GOVNANCE_MODULE}
build_module ${INCOME_MODULE}
build_module ${KERNEL_MODULE}
build_module ${MULTISIG_MODULE}
build_module ${MULTISIGFACTORY_MODULE}
build_module ${ROLEMANAGE_MODULE}
build_module ${ROUTE_MODULE}
build_module ${USERS_MODULE}
