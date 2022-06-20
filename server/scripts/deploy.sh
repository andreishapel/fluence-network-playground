#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

SERVICE=$1
SECRET_KEY=${2-Pfw9nfmCAVVONZAIZAnzyWMNTnKgn+3u3J/p7xSVxw8=}
ADDRESS=${3-/dns4/kras-04.fluence.dev/tcp/19001/wss/p2p/12D3KooWFEwNWcHqi9rtsmDhsYcDbRUCDXH84RC4FW6UfsFWaoHi}

aqua remote deploy_service \
  --addr $ADDRESS \
  --sk $SECRET_KEY \
  --config-path configs/deployment.json \
  --service $SERVICE
