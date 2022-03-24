FROM ubuntu:latest

# Install all the stuff as root
ARG STOCKFISH_FILENAME=stockfish_14.1_linux_x64
ARG STOCKFISH_URL=https://stockfishchess.org/files/${STOCKFISH_FILENAME}.zip

RUN apt-get update && apt upgrade -y
RUN apt-get install sudo curl unzip -y

# Run all the stuff as the sf user
RUN useradd -m -G root sf
USER sf
WORKDIR /home/sf/

RUN curl ${STOCKFISH_URL} -o ~/stockfish.zip
RUN unzip ~/stockfish.zip
RUN rm ~/stockfish.zip
RUN cp ~/${STOCKFISH_FILENAME}/${STOCKFISH_FILENAME} ~/${STOCKFISH_FILENAME}/stockfish
RUN chmod 777 ~/${STOCKFISH_FILENAME}/stockfish
ENV PATH="/home/sf/${STOCKFISH_FILENAME}:${PATH}"