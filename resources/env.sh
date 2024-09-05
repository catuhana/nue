#!/bin/sh
# nue shell setup
case ":${PATH}:" in
*:"$HOME/.nue/node/bin":*) ;;
*)
  export PATH="$PATH:$HOME/.nue/node/bin"
  ;;
esac
