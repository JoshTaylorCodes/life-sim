#!/bin/zsh
# #1690 trial: trial.sh <name> [--param k=v ...]. One long run at a time.
cd /Users/joshtaylor/CodingProjects/life-sim-batch/tidepool || exit 1
N=$1; shift
OUT=/Users/joshtaylor/CodingProjects/life-sim-batch/trials/$N.log
H=target/release/tidepool-headless
{
echo "trial $N head $(git rev-parse --short HEAD) params: $* start $(date) $(uptime)"
echo "== gate-health"; nice $H gate-health "$@"; echo "exit $?"
echo "== probe-diet 144k"; nice $H probe-diet --seeds 1,4,5,6,8 --ticks 144000 --every 12000 "$@"; echo "exit $?"
echo "end $(date)"
} > $OUT 2>&1
python3 /Users/joshtaylor/CodingProjects/life-sim-batch/score.py $OUT
