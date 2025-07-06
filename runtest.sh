#!/bin/bash

echo "begin runtest.sh"

echo "rustup update"
rustup update

cd malloc-test
echo "$(date) before cargo build"
cargo build --release
echo "$(date) after cargo build"

echo "run ./target/release/malloc-test"
./target/release/malloc-test &

PID=$!
sleep 1

echo "PID = $PID"

i=0

while [ $i -lt 3600 ]; do
  sleep 1

  RSS_KB=$(ps -eo pid,user,rss,time -q $PID | tail -1 | awk '{print $3}' )

  RSS_MB=$(bc <<< "scale=1; $RSS_KB / 1000")

  CPU_TIME=$(ps -eo pid,user,rss,time -q $PID | tail -1 | awk '{print $4}' )

  NUM_THREADS=$(ps -eLf -q $PID | grep -v PID | wc -l)

  echo
  echo "$(date) i=$i PID=$PID RSS_KB=$RSS_KB RSS_MB=$RSS_MB CPU_TIME=$CPU_TIME NUM_THREADS=$NUM_THREADS"
  
  i=$((i+1))
done


echo "end runtest.sh"