FROM python:3.11-slim

RUN apt-get clean && apt-get -y update

RUN apt-get -y install nginx \
    && apt-get -y install python3-dev \
    && apt-get -y install build-essential

WORKDIR /srv/flask_app/server

# https://dev.to/ajeetraina/boost-your-docker-workflow-introducing-docker-init-for-python-developers-3mh6
# https://github.com/tiangolo/uwsgi-nginx-flask-docker/issues/66
# ARG UID=10001
# RUN adduser \
#     --disabled-password \
#     --gecos "" \
#     --home "/nonexistent" \
#     --shell "/sbin/nologin" \
#     --no-create-home \
#     --uid "${UID}" \
#     appuser

COPY requirements.txt ./
RUN python -m pip install --no-cache-dir -r requirements.txt

RUN opentelemetry-bootstrap --action=install

# USER appuser

COPY . /srv/flask_app
COPY ./server/nginx.conf /etc/nginx

RUN chmod +x ./start.sh
CMD ["./start.sh"]

# CMD [ "opentelemetry-instrument", "python", "./main.py" ]