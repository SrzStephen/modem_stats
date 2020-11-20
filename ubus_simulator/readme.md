# Ubus simulator
Unsurprisingly, my (and probably your) development machine doesn't have OpenWRT installed.

If it did have openWRT installed then it's unlikely to have the same responses/calls, so this just replays 
some known good files, and implements enough ubus to develop against as a python ```console_script```.

# Installation
```pip3 install .```
# Warning
While I've made efforts to avoid this installing on systems with the legitimate ubus binary in your current ```PATH```,
you should probably avoid trying to install it on anything with OpenWRT or a system you're not familiar with.

The guard in setup.py:
```python
from distutils.spawn import find_executable
from subprocess import Popen, PIPE
if find_executable('ubus'):
    process = Popen('ubus', stdout=PIPE)
    stdout, _ = process.communicate(timeout=10)
    if process.returncode !=0:
        raise OSError(f"trying to call 'ubus' gave error code {process.returncode}, can't determine whether you have "
                      f"ubus installed or not.")
    if "This isn't the real ubus" not in stdout.__str__():
        raise OSError(f"You may have the real ubus installed. Exiting for safety \n"
                      f"{stdout.__str__()}")

```

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




