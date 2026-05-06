#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
command=${1:-help}
if [[ $# -gt 0 ]]; then
  shift
fi

sandbox_name=${GCODE_ONBOARDING_SANDBOX:-default}
sandbox_root_default="$repo_root/.tmp/onboarding/$sandbox_name"
sandbox_root=${GCODE_ONBOARDING_DIR:-$sandbox_root_default}
gcode_home="$sandbox_root/home"
runtime_dir="$sandbox_root/runtime"
mobile_socket="$runtime_dir/gcode-mobile-sim.sock"

ensure_dirs() {
  mkdir -p "$gcode_home" "$runtime_dir"
}

run_in_sandbox() {
  ensure_dirs
  (
    cd "$repo_root"
    env \
      GCODE_HOME="$gcode_home" \
      GCODE_RUNTIME_DIR="$runtime_dir" \
      "$@"
  )
}


print_usage() {
  cat <<EOF
Usage: $(basename "$0") <command> [args...]

Commands:
  env                    Print the sandbox environment exports
  status                 Show sandbox paths and current contents
  reset                  Delete the sandbox entirely
  shell                  Open a clean shell with sandbox env vars set
  gcode [args...]        Run gcode inside the sandbox
  auth-status            Run 'gcode auth status' inside the sandbox
  fresh [args...]        Reset sandbox, then launch gcode with args
  login <provider> ...   Run 'gcode --provider <provider> login ...' in sandbox
  mobile-start [scenario]
                         Start gcode-mobile-sim in background (default: onboarding)
  mobile-serve [scenario]
                         Run gcode-mobile-sim in foreground (default: onboarding)
  mobile-status          Show mobile simulator status
  mobile-state           Show full mobile simulator state
  mobile-reset           Reset the mobile simulator back to its initial scenario
  mobile-log             Show mobile simulator transition log
  help                   Show this help

Environment overrides:
  GCODE_ONBOARDING_SANDBOX   Sandbox name (default: default)
  GCODE_ONBOARDING_DIR       Explicit sandbox directory

Examples:
  $(basename "$0") fresh
  $(basename "$0") login openai
  $(basename "$0") auth-status
  $(basename "$0") mobile-start onboarding
  $(basename "$0") mobile-status
EOF
}

print_env() {
  ensure_dirs
  cat <<EOF
export GCODE_HOME="$gcode_home"
export GCODE_RUNTIME_DIR="$runtime_dir"
EOF
}

status() {
  ensure_dirs
  echo "Sandbox name: $sandbox_name"
  echo "Sandbox root: $sandbox_root"
  echo "GCODE_HOME:   $gcode_home"
  echo "RUNTIME_DIR:  $runtime_dir"
  echo

  if [[ -d "$gcode_home" ]]; then
    echo "Home contents:"
    find "$gcode_home" -maxdepth 3 \( -type f -o -type d \) | sed "s#^$sandbox_root#.#" | sort
  fi
  echo

  if [[ -S "$mobile_socket" ]]; then
    echo "Mobile simulator socket: $mobile_socket"
  else
    echo "Mobile simulator socket: not running"
  fi
}

reset() {
  rm -rf "$sandbox_root"
  echo "Removed onboarding sandbox: $sandbox_root"
}

open_shell() {
  ensure_dirs
  echo "Opening sandbox shell"
  echo "  GCODE_HOME=$gcode_home"
  echo "  GCODE_RUNTIME_DIR=$runtime_dir"
  env GCODE_HOME="$gcode_home" GCODE_RUNTIME_DIR="$runtime_dir" bash --noprofile --norc
}

run_gcode() {
  local binary_path="$repo_root/target/debug/gcode"
  if [[ -x "$binary_path" ]]; then
    run_in_sandbox "$binary_path" "$@"
  else
    run_in_sandbox cargo run --bin gcode -- "$@"
  fi
}

run_mobile_sim() {
  local binary_path="$repo_root/target/debug/gcode-mobile-sim"
  if [[ -x "$binary_path" ]]; then
    run_in_sandbox "$binary_path" "$@"
  else
    run_in_sandbox cargo run -p gcode-mobile-sim -- "$@"
  fi
}

scenario_arg() {
  if [[ $# -gt 0 ]]; then
    printf '%s' "$1"
  else
    printf 'onboarding'
  fi
}

case "$command" in
  env)
    print_env
    ;;
  status)
    status
    ;;
  reset)
    reset
    ;;
  shell)
    open_shell
    ;;
  gcode)
    run_gcode "$@"
    ;;
  auth-status)
    run_gcode auth status
    ;;
  fresh)
    reset
    run_gcode "$@"
    ;;
  login)
    if [[ $# -lt 1 ]]; then
      echo "login requires a provider, for example: $(basename "$0") login openai" >&2
      exit 1
    fi
    provider=$1
    shift
    run_gcode --provider "$provider" login "$@"
    ;;
  mobile-start)
    scenario=$(scenario_arg "$@")
    run_mobile_sim start --scenario "$scenario"
    ;;
  mobile-serve)
    scenario=$(scenario_arg "$@")
    run_mobile_sim serve --scenario "$scenario"
    ;;
  mobile-status)
    run_mobile_sim status
    ;;
  mobile-state)
    run_mobile_sim state
    ;;
  mobile-reset)
    run_mobile_sim reset
    ;;
  mobile-log)
    run_mobile_sim log
    ;;
  help|-h|--help)
    print_usage
    ;;
  *)
    echo "Unknown command: $command" >&2
    echo >&2
    print_usage >&2
    exit 1
    ;;
esac
