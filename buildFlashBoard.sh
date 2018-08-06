#!/bin/sh -e
PROJECT=$(pwd)

#Building
xargo build --release

#Deploying/Flashing
openocd -f board/stm32f4discovery_Script.cfg \
  -c "init" \
  -c "reset init" \
  -c "flash write_image erase $PROJECT/target/thumbv7em-none-eabihf/release/emb1" \
  -c "reset halt" \
  -c "resume" \
  -c "shutdown"
