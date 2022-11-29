FROM node:14-alpine
WORKDIR /app
COPY . .


ENV PORT 8080
ENV HOST 127.0.0.1
EXPOSE 8080

RUN ls
RUN npm i -g http-server

CMD http-server out