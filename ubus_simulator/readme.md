# Ubus simulator
Unsurprisingly, my (and probably your) development machine doesn't have OpenWRT installed.

If it did have openWRT installed then it's unlikely to have the same responses/calls, so this just replays 
some known good files, and implements enough ubus to develop against as a python ```console_script```.

# Installation
```pip3 install -e .```

# Usage
If you're not trying to test ```modemstats``` then you probably shouldn't be calling this
```
➜  ubus
Usage: ubus [OPTIONS] COMMAND [ARGS]...

  This isn't the real ubus. It's a simulator

Options:
  --help  Show this message and exit.

Commands:
  mobiled.radio
  network.device
  system
  wireless.radio.stats


```




