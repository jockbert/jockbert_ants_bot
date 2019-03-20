#!/bin/sh

RUST_BACKTRACE=1 cargo build && \
cp target/debug/jockbot_ants_bot tools && \
cd tools && \
python ./playgame.py --turntime=5000 -l game_logs -m maps/maze/maze_p02_01.map -I -e -v --strict \
"./jockbot_ants_bot $@" "python sample_bots/python/HunterBot.py" && \
cd ..