#!/bin/bash

echo "begin runtest.sh"

echo "cc --version"
cc --version

echo "cc -pthread test.c"
cc -pthread test.c

echo "run ./a.out"
./a.out &

PID=$!
sleep 1

echo "PID = $PID"

i=0

while [ $i -lt 3600 ]; do
  sleep 1

  RSS_KB=$(ps -eo pid,user,rss,time -q $PID | tail -1 | awk '{print $3}' )

  RSS_MB=$(bc <<< "scale=1; $RSS_KB / 1000")

  CPU_TIME=$(ps -eo pid,user,rss,time -q $PID | tail -1 | awk '{print $4}' )

  NUM_THREADS=$(ps -eLf -q $API_PID | grep -v PID | wc -l)

  echo
  echo "$(date) i=$i RSS_KB=$RSS_KB RSS_MB=$RSS_MB CPU_TIME=$CPU_TIME NUM_THREADS=$NUM_THREADS"
  
  i=$((i+1))
done


echo "end runtest.sh"