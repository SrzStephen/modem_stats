from setuptools import setup, find_packages
from os import path
from distutils.spawn import find_executable
from subprocess import Popen, PIPE

here = path.abspath(path.dirname(__file__))
with open(path.join(here, 'readme.md'), encoding='utf-8') as f:
    long_description = f.read()

if find_executable('ubus'):
    process = Popen('ubus', stdout=PIPE)
    stdout, _ = process.communicate(timeout=10)
    if process.returncode !=0:
        raise OSError(f"trying to call 'ubus' gave error code {process.returncode} can't determine whether you have ubus installed or not.")
    if "This isn't the real ubus" not in stdout.__str__():
        raise OSError(f"You may have the real ubus installed. Exiting for safety \n {stdout.__str__()}")



setup(
    name="ubus-simulator",
    version="0.0.1",
    description="Emulator for parts of OpenWRTs ubus to help develop against.",
    long_description_content_type="text/markdown",
    url="TODO",
    author="Stephen Mott",
    classifiers=["Development Status ::3 - Alpha",
                 "Intended Audience :: Developers",
                 "Programming Language :: Python :: 3.7"],
    package_dir={'': 'src'},
    packages=find_packages(where='src'),
    python_requires=">=3.5",
    platforms="linux",
    entry_points={'console_scripts': ['ubus=ubus_simulator.cli:ubus'
                                      ]},
    include_package_data=True,
    package_data={
        "":["data/*.json"]
    },
    install_requires=['click','pytest']
)
