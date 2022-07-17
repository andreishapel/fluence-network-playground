#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

SERVICE=$1
SECRET_KEY=${2-Pfw9nfmCAVVONZAIZAnzyWMNTnKgn+3u3J/p7xSVxw8=}
ADDRESS=${3-/dns4/kras-00.fluence.dev/tcp/19001/wss/p2p/12D3KooWR4cv1a8tv7pps4HH6wePNaK6gf1Hww5wcCMzeWxyNw51}

aqua remote deploy_service \
  --addr $ADDRESS \
  --sk $SECRET_KEY \
  --config-path configs/deployment.json \
  --service $SERVICE
