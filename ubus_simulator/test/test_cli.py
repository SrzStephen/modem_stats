from unittest import TestCase
from click.testing import CliRunner
from json import loads
from ubus_simulator import ubus
from typing import List
from subprocess import Popen
from subprocess import PIPE
"""
data["mobile"] = json::parse( & getStdout(vec!["mobiled.radio", "signal_quality"])).unwrap();
data["system"] = json::parse( & getStdout(vec!["system", "info"])).unwrap();
data["wireless"] = json::parse( & getStdout(vec!["wireless.radio.stats", "get"])).unwrap();
data["network"] = json::parse( & getStdout(vec!["network.device", "status"])).unwrap();
"""


class TestCLI(TestCase):
    def setUp(self) -> None:
        self.runner = CliRunner()

    def test_ubus_help(self):
        self.__run_command([])

    def __run_command(self, args=List[str]):
        result = self.runner.invoke(ubus, args)
        self.assertEqual(result.exit_code, 0, msg=f"failed to run ubus {args}, got {result.stdout}")
        if len(args) == 0:
            self.assertIn("This isn't the real ubus. It's a simulator", result.stdout)

        else:
            stdout = loads(result.stdout)
            self.assertIsInstance(stdout, dict)
            self.assertGreater(len(stdout.keys()), 0)

    def test_mobile(self):
        self.__run_command(['call', 'mobiled.radio', 'signal_quality'])

    def test_wireless_radio(self):
        self.__run_command(['call', 'wireless.radio.stats', 'get'])

    def test_system(self):
        self.__run_command(['call', 'system', 'info'])

    def test_network_device(self):
        self.__run_command(['call', 'network.device', 'status'])
    def test_system_command(self):
        """all tests before this tested the python command.. did it pip install as a proper command?"""
        process = Popen(['ubus'],stdout=PIPE)
        stdout, _ = process.communicate()
        self.assertEqual(process.returncode,0)
        self.assertIn("This isn't the real ubus. It's a simulator",stdout.__str__())
