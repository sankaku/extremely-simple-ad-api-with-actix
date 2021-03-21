FROM redis:6.2.1-alpine3.13
COPY ./redis.conf /usr/local/etc/redis/redis.conf
RUN mkdir /run/redis
RUN chown -R redis:redis /run/redis

USER redis
CMD [ "redis-server", "/usr/local/etc/redis/redis.conf" ]