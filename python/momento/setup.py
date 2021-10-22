import setuptools
import os

version = os.getenv("PYPI_MOMENTO_WIRE_TYPE_VERSION")
assert version != None

# version is the only dynamic configuration
setuptools.setup(
    version=version,
)