import click
from json import dumps, load
from pathlib import Path
data_dir = Path(__file__).parent.absolute() / 'data/'

def load_file(file_path:str) -> str:
    with open(data_dir / file_path) as fp:
        return dumps(load(fp))
        



@click.group()
def ubus():
    """This isn't the real ubus. It's a simulator"""
    pass

@ubus.group(name="call")
def call():
    """This isn't the real ubus. It's a simulator"""
    pass

@call.group(name="system")
def system():
    pass


@call.group(name="wireless.radio.stats")
def wireless_radio_stats():
    pass


@call.group(name="network.device")
def network_device():
    pass


@call.group(name="mobiled.radio")
def mobiled_radio():
    pass


@mobiled_radio.command(name="signal_quality")
def mobiled_signal():
    print(load_file('mobile.json'))


@wireless_radio_stats.command(name="get")
def wireless_radio_stats():
    print(load_file('wireless.json'))


@network_device.command(name="status")
def network_device_status():
    print(load_file('network.json'))


@system.command(name="info")
def system_info():
    print(load_file('system.json'))


if __name__ == "__main__":
    ubus()
