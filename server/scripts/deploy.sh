#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

SERVICE=$1
SECRET_KEY=${2-NAB5rGwT4qOEB+6nLQawkTfCOV2eiFSjgQK8bfEdZXY=}
ADDRESS=${3-/ip4/127.0.0.1/tcp/9990/ws/p2p/12D3KooWHBG9oaVx4i3vi6c1rSBUm7MLBmyGmmbHoZ23pmjDCnvK}

aqua remote deploy_service \
  --addr $ADDRESS \
  --sk $SECRET_KEY \
  --config-path configs/deployment.json \
  --service $SERVICE
