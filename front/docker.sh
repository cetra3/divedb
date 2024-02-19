#! /usr/bin/env bash
set -e

while true
do
  yarn build
  # ISR one day... https://github.com/sveltejs/kit/issues/661
  # Make sure we have the file system notify lock
  touch build/notify.lock
  inotifywait build/notify.lock
done
