import click
from .data import network_data, wireless_data, system_data, mobile_data
from json import dumps


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
    print(dumps(mobile_data))


@wireless_radio_stats.command(name="get")
def wireless_radio_stats():
    print(dumps(wireless_data))


@network_device.command(name="status")
def network_device_status():
    print(dumps(network_data))


@system.command(name="info")
def system_info():
    print(dumps(system_data))


if __name__ == "__main__":
    ubus()
