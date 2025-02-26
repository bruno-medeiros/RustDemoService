FROM node:18.17.0-alpine3.18
WORKDIR /src
COPY snippets .
RUN echo 'Hello, Compiler!'
CMD ["echo", "Hello, World"]