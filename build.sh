#!/bin/bash


distro=$(awk -F= '$1 == "ID_LIKE" { print $2 }' /etc/*-release)

if ! [ -x "$(command -v cargo)" ]; then
  echo 'Error: rust compiler is not installed.' >&2

    echo 'installing rust compiler..'
    #curls the rust compiler

    curl https://sh.rustup.rs -sSf | sh -s -- -y

    . $HOME/.cargo/env

fi


[ ! -d  ~/.punch ] && \

     mkdir -p ~/.punch/trash && mkdir ~/.punch/bin && echo "~/.punch created in home"


cargo build  --release \
 && echo "succefully compiled"

mv ./target/release/punch ~/.punch/bin/

{
    printf 'alias punch="~/.punch/bin/punch"' >> ~/.zshrc \
    && printf 'alias punch="~/.punch/bin/punch"' >> ~/.bashrc \
    && . ~/.zshrc \
    && . ~/.bashrc

}||{
     printf 'alias punch="~/.punch/bin/punch"' >> ~/.bashrc \
     && . ~/.bashrc
}||{
     printf 'alias punch="~/.punch/bin/punch"' >> ~/.zshrc \
     && . ~/.zshhrc
}||{
   echo "failed: if you are on debian cc build essentials must be installed, run :" \
     \
     && echo 'sudo apt update && sudo apt upgrade && sudo apt-get install build-essential' \
     \
     && echo 'after running, try building again'
}
