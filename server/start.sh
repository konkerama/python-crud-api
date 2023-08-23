#!/usr/bin/env bash
service nginx start
cd ../app || exit
uwsgi --ini uwsgi.ini