#!/bin/sh

set -ex


cd /usr/local/bin


# Install Conventional Commits - Cocogitto (`cog`)
CURRENT_REPO="cocogitto/cocogitto"
CURRENT_VERSION=$(curl -sSfL https://api.github.com/repos/${CURRENT_REPO}/releases | jq '.[0].tag_name' | tr -d '"')
curl -sSfL https://github.com/${CURRENT_REPO}/releases/download/${CURRENT_VERSION}/cocogitto-${CURRENT_VERSION}-x86_64-unknown-linux-musl.tar.gz |  zcat - | tar xvf - cog


# Install Just (`just`)
CURRENT_REPO="casey/just"
CURRENT_VERSION=$(curl -sSfL https://api.github.com/repos/${CURRENT_REPO}/releases | jq '.[0].tag_name' | tr -d '"')
curl -sSfL https://github.com/${CURRENT_REPO}/releases/download/${CURRENT_VERSION}/just-${CURRENT_VERSION}-x86_64-unknown-linux-musl.tar.gz |  zcat - | tar xvf - just


# Install Difftastic (`difft`)
CURRENT_REPO="Wilfred/difftastic"
CURRENT_VERSION=$(curl -sSfL https://api.github.com/repos/${CURRENT_REPO}/releases | jq '.[0].tag_name' | tr -d '"')
curl -sSfL https://github.com/${CURRENT_REPO}/releases/download/${CURRENT_VERSION}/difft-x86_64-unknown-linux-gnu.tar.gz |  zcat - | tar xvf - difft


# Install Watchexec (`watchexec`)
CURRENT_REPO="watchexec/watchexec"
CURRENT_VERSION="$(curl -sSfL https://api.github.com/repos/${CURRENT_REPO}/releases | jq '.[0].tag_name' | tr -d '"' | cut -c 2-)"
curl -sSfL https://github.com/${CURRENT_REPO}/releases/download/v${CURRENT_VERSION}/watchexec-${CURRENT_VERSION}-x86_64-unknown-linux-musl.tar.xz | tar -Jxvf - "watchexec-${CURRENT_VERSION}-x86_64-unknown-linux-musl/watchexec"
mv ./watchexec-${CURRENT_VERSION}-x86_64-unknown-linux-musl/watchexec ./
rm -rf watchexec-${CURRENT_VERSION}-x86_64-unknown-linux-musl


# Install Cargo audit (`cargo-audit`)
CURRENT_REPO="rustsec/rustsec"
CURRENT_TAG_NAME=$(curl -sSfL https://api.github.com/repos/${CURRENT_REPO}/releases | jq 'map(select (.tag_name | startswith("cargo-audit"))) | first | .tag_name' | tr -d '"')
CURRENT_VERSION=$(echo ${CURRENT_TAG_NAME} | cut -d '/' -f 2)
curl -sSfL https://github.com/${CURRENT_REPO}/releases/download/cargo-audit/${CURRENT_VERSION}/cargo-audit-x86_64-unknown-linux-musl-${CURRENT_VERSION}.tgz |  zcat - | tar xvf - "cargo-audit-x86_64-unknown-linux-musl-${CURRENT_VERSION}/cargo-audit"
mv ./cargo-audit-x86_64-unknown-linux-musl-${CURRENT_VERSION}/cargo-audit ./
rm -rf ./cargo-audit-x86_64-unknown-linux-musl-${CURRENT_VERSION}

# Install Cargo audit (`cargo-deny`)
CURRENT_REPO="EmbarkStudios/cargo-deny"
CURRENT_VERSION=$(curl -sSfL https://api.github.com/repos/${CURRENT_REPO}/releases | jq '.[0].tag_name' | tr -d '"')
curl -sSfL https://github.com/${CURRENT_REPO}/releases/download/${CURRENT_VERSION}/cargo-deny-${CURRENT_VERSION}-x86_64-unknown-linux-musl.tar.gz |  zcat - | tar xvf - "cargo-deny-${CURRENT_VERSION}-x86_64-unknown-linux-musl/cargo-deny"
mv ./cargo-deny-${CURRENT_VERSION}-x86_64-unknown-linux-musl/cargo-deny ./
rm -rf ./cargo-deny-${CURRENT_VERSION}-x86_64-unknown-linux-musl

# Install wasm-bindgen (`wasm-bindgen`)
CURRENT_REPO="rustwasm/wasm-bindgen"
CURRENT_VERSION=$(curl -sSfL https://api.github.com/repos/${CURRENT_REPO}/releases | jq '.[0].tag_name' | tr -d '"')
curl -sSfL https://github.com/${CURRENT_REPO}/releases/download/${CURRENT_VERSION}/wasm-bindgen-${CURRENT_VERSION}-x86_64-unknown-linux-musl.tar.gz | zcat - | tar xvf - "wasm-bindgen-${CURRENT_VERSION}-x86_64-unknown-linux-musl/"
mv wasm-bindgen-${CURRENT_VERSION}-x86_64-unknown-linux-musl/wasm-bindgen ./
mv wasm-bindgen-${CURRENT_VERSION}-x86_64-unknown-linux-musl/wasm-bindgen-test-runner ./
mv wasm-bindgen-${CURRENT_VERSION}-x86_64-unknown-linux-musl/wasm2es6js ./
rm -rf ./wasm-bindgen-${CURRENT_VERSION}-x86_64-unknown-linux-musl/


# Install wasm-pack (`wasm-pack`)
CURRENT_REPO="rustwasm/wasm-pack"
CURRENT_VERSION=$(curl -sSfL https://api.github.com/repos/${CURRENT_REPO}/releases | jq '.[0].tag_name' | tr -d '"')
curl -sSfL https://github.com/${CURRENT_REPO}/releases/download/${CURRENT_VERSION}/wasm-pack-${CURRENT_VERSION}-x86_64-unknown-linux-musl.tar.gz |  zcat - | tar xvf - "wasm-pack-${CURRENT_VERSION}-x86_64-unknown-linux-musl/"
mv wasm-pack-${CURRENT_VERSION}-x86_64-unknown-linux-musl/wasm-pack ./
rm -rf ./wasm-pack-${CURRENT_VERSION}-x86_64-unknown-linux-musl/


# Install cargo generate (`cargo-generate`) 
CURRENT_REPO="cargo-generate/cargo-generate"
CURRENT_VERSION=$(curl -sSfL https://api.github.com/repos/${CURRENT_REPO}/releases | jq '.[0].tag_name' | tr -d '"')
curl -sSfL https://github.com/${CURRENT_REPO}/releases/download/${CURRENT_VERSION}/cargo-generate-${CURRENT_VERSION}-x86_64-unknown-linux-musl.tar.gz |  zcat - | tar xvf - "cargo-generate"
chmod +x cargo-generate


# Install WASM tooling: `trunk`. 
CURRENT_REPO="thedodd/trunk"
CURRENT_VERSION=$(curl -sSfL https://api.github.com/repos/${CURRENT_REPO}/releases | jq '.[0].tag_name' | tr -d '"')
curl -sSfL https://github.com/${CURRENT_REPO}/releases/download/${CURRENT_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz |  zcat - | tar xvf - "trunk"
chmod +x trunk

