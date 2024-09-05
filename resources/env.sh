# nue
case ":${PATH}:" in
*:"$HOME/.nue/bin":*) ;;
*)
  export PATH="$HOME/.nue/node/bin:$PATH"
  ;;
esac
