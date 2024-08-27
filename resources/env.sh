# Append this file to your shell profile to
# make Node and it's tools usable from the command line.
#
# nue
case ":${PATH}:" in
*:"$HOME/.nue/bin":*) ;;
*)
  export PATH="$HOME/.nue/bin:$PATH"
  ;;
esac
