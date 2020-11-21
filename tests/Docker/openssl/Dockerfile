FROM frapsoft/openssl

RUN mkdir /export/
COPY commands.sh /export/commands.sh
WORKDIR /export/
RUN chmod u+x /export/commands.sh
ENTRYPOINT ["sh", "commands.sh" ]