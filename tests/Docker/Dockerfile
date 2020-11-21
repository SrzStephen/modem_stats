FROM eclipse-mosquitto:1.6.12
RUN mkdir certs
COPY openssl/ca.crt /certs/ca.crt
COPY openssl/server.key /certs/server.key
COPY openssl/server.crt /certs/server.crt
COPY mosq.conf /mosquitto/config/mosquitto.conf
ENTRYPOINT [ "mosquitto","-c","/mosquitto/config/mosquitto.conf" ]
