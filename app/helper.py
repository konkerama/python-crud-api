import configparser
import os
from pathlib import Path

# ENV = os.environ['ENV']
ENV = os.getenv("ENV", "dev")


# Method to read config file settings
def read_config():
    config = configparser.ConfigParser()
    config_path = Path(__file__).resolve().parent / "config" / f"{ENV}.ini"
    config.read(config_path)
    return config
