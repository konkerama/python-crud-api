import configparser

# ENV = os.environ['ENV']
ENV="dev"

# Method to read config file settings
def read_config():
    config = configparser.ConfigParser()
    config.read(f'./config/{ENV}.ini')
    return config