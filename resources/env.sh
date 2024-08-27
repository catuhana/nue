# nue
case ":${PATH}:" in
*:"$HOME/.nue/bin":*) ;;
*)
  export PATH="$HOME/.nue/bin:$PATH"
  ;;
esac
