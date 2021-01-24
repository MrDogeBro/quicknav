pub fn init() {
    let shell = r#"function __quicknav_cd() {
  cd "$@"
}

function __quicknav_nav() {
  if [ "$#" -eq 0 ]; then
    quicknav --help
  else
    local __quicknav_result
    __quicknav_result="$(quicknav get "$@")"

    if [[ $__quicknav_result == /* ]]; then
      __quicknav_cd "$__quicknav_result"
    else
      echo "\033[0;31m$__quicknav_result"
    fi
  fi
}

function nav() {
  __quicknav_nav "$@"
}"#;

    println!("{}", shell)
}
