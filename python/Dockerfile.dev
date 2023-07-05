FROM python:3.10-slim

RUN apt-get clean && apt-get -y update

RUN apt-get -y install nginx \
    && apt-get -y install python3-dev \
    && apt-get -y install build-essential

WORKDIR /srv/flask_app/server

COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt

RUN opentelemetry-bootstrap --action=install

COPY . /srv/flask_app
COPY ./server/nginx.conf /etc/nginx

RUN chmod +x ./start.sh
CMD ["./start.sh"]

# CMD [ "opentelemetry-instrument", "python", "./main.py" ]