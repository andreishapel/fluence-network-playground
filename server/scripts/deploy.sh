#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

aqua remote deploy_service \
  --addr /dns4/kras-04.fluence.dev/tcp/19001/wss/p2p/12D3KooWFEwNWcHqi9rtsmDhsYcDbRUCDXH84RC4FW6UfsFWaoHi \
  --sk Pfw9nfmCAVVONZAIZAnzyWMNTnKgn+3u3J/p7xSVxw8= \
  --config-path configs/deployment.json \
  --service crypto
