#!/bin/bash

start(){
    cp -r assets/dist/* priv/
    cargo run
}
case $1 in
start) start;;
*) ;;
esac