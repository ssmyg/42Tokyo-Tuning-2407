#!/bin/sh

if [ -f ./webapp/log/mysql/slow-query.log ]; then
  mv ./webapp/log/mysql/slow-query.log ./webapp/log/mysql/slow-query.log.`date +%Y%m%d-%H%M%S`
fi
if [ -f ./webapp/log/nginx/access.log ]; then
  mv ./webapp/log/nginx/access.log ./webapp/log/nginx/access.log.`date +%Y%m%d-%H%M%S`
fi