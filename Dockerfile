FROM node:14-stretch-slim

COPY . .

WORKDIR client
RUN npm update
RUN npm install
RUN npm rebuild node-sass
RUN npm run build
