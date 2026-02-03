#!/bin/bash

if [[ ${RISCV_SAIL} == "" ]]
then
    echo "Please define RISCV_SAIL pointing the sources of https://github.com/riscv/sail-riscv/"
    exit 1
fi

config=${RISCV_SAIL}/build/config/rv64d_v128_e64.json

if [[ ! -f ${config} ]]
then
    echo "${config} not found, make sure you run ./build_simulators.sh in ${RISCV_SAIL}"
fi

set -e
set -x

eval $(opam env)

sail --project ${RISCV_SAIL}/model/riscv.sail_project \
     --all-modules \
     --output-sail \
     --config ${config} > dump.sail
