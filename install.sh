#!/usr/bin/env bash

# Luna — one-line installer.

#

# Usage:

#   curl -fsSL https://raw.githubusercontent.com/akhwasim/luna/main/install.sh | bash

#

# What it does:

#   1. Detects OS (linux/darwin) and architecture (x86_64/aarch64)

#   2. Downloads the latest release binary from GitHub Releases

#   3. Installs to ~/.local/bin/luna (no sudo needed)

#   4. Sets up ~/.luna/ for config and memory

#   5. Falls back to `cargo install --git` if no release is available yet

#

# After install, just run `luna` to start. First launch walks you

# through picking an AI provider.


set -e


# --- Configuration ---

REPO="akhwasim/luna"

INSTALL_DIR="${LUNA_INSTALL_DIR:-$HOME/.local/bin}"

BINARY_NAME="luna"


# --- Color output (only if stdout is a TTY) ---

if [ -t 1 ]; then

    BLUE='\033[1;34m'

    GREEN='\033[1;32m'

    YELLOW='\033[1;33m'

    RED='\033[1;31m'

    RESET='\033[0m'

else

    BLUE=''; GREEN=''; YELLOW=''; RED=''; RESET=''

fi


info()  { printf "${BLUE}==>${RESET} %s\n" "$1"; }

ok()    { printf "${GREEN}✓${RESET} %s\n" "$1"; }

warn()  { printf "${YELLOW}!${RESET} %s\n" "$1"; }

fail()  { printf "${RED}✗${RESET} %s\n" "$1" >&2; exit 1; }


# --- Detect OS and architecture ---

detect_target() {

    local os arch


    case "$(uname -s)" in

        Linux)  os="unknown-linux-gnu" ;;

        Darwin) os="apple-darwin" ;;

        *)      fail "Unsupported OS: $(uname -s). Luna currently ships linux + macOS binaries." ;;

    esac


    case "$(uname -m)" in

        x86_64)  arch="x86_64" ;;

        aarch64) arch="aarch64" ;;

        arm64)   arch="aarch64" ;;  # macOS Apple Silicon

        *)       fail "Unsupported architecture: $(uname -m)" ;;

    esac


    echo "${arch}-${os}"

}


# --- Find the latest release version ---

get_latest_version() {

    # Try GitHub API. If it fails (no releases yet, no network), return empty.

    if command -v curl >/dev/null 2>&1; then

        curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \

            | grep -oE '"tag_name": *"v[^"]*"' \

            | head -1 \

            | sed -E 's/.*"v([^"]+)".*/\1/' \

            || true

    fi

}


# --- Download and install a prebuilt binary ---

install_from_release() {

    local version="$1"

    local target="$2"

    local archive="luna-${target}.tar.gz"

    local url="https://github.com/${REPO}/releases/download/v${version}/${archive}"


    info "Downloading Luna v${version} for ${target}..."

    local tmpdir

    tmpdir="$(mktemp -d)"

    trap 'rm -rf "$tmpdir"' EXIT


    if ! curl -fsSL -o "${tmpdir}/${archive}" "$url"; then

        warn "Prebuilt binary not available for ${target} at v${version}"

        return 1

    fi


    info "Extracting..."

    tar -xzf "${tmpdir}/${archive}" -C "$tmpdir"


    info "Installing to ${INSTALL_DIR}/..."

    mkdir -p "$INSTALL_DIR"

    mv "${tmpdir}/luna" "${INSTALL_DIR}/luna"

    chmod +x "${INSTALL_DIR}/luna"


    ok "Luna v${version} installed to ${INSTALL_DIR}/luna"

}


# --- Build from source via cargo ---

install_from_source() {

    if ! command -v cargo >/dev/null 2>&1; then

        fail "cargo not found. Install Rust from https://rustup.rs then re-run this script."

    fi


    info "Building Luna from source (this takes 2-3 minutes)..."

    cargo install --git "https://github.com/${REPO}.git" --locked --force


    ok "Luna built and installed via cargo"

}


# --- Set up ~/.luna/ directory ---

setup_luna_dir() {

    local luna_dir="$HOME/.luna"

    if [ ! -d "$luna_dir" ]; then

        mkdir -p "$luna_dir"

        ok "Created ${luna_dir} for config and memory"

    fi

}


# --- Verify install and print next steps ---

verify_and_next() {

    if command -v "$BINARY_NAME" >/dev/null 2>&1; then

        ok "Luna is installed and on your PATH"

    elif [ -x "${INSTALL_DIR}/luna" ]; then

        warn "Luna is at ${INSTALL_DIR}/luna but not on your PATH"

        warn "Add this to your shell profile (~/.bashrc, ~/.zshrc, etc.):"

        echo ""

        echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""

        echo ""

        warn "Then restart your shell or run: source ~/.bashrc"

    else

        fail "Luna binary not found after install. Something went wrong."

    fi


    echo ""

    info "Next steps:"

    echo "    1. Run: luna"

    echo "    2. Pick an AI provider (Groq is free + fast)"

    echo "    3. Paste your API key (or pick Ollama for offline)"

    echo "    4. Type /luna to ask anything terminal-related"

    echo ""

    info "Documentation: https://github.com/${REPO}"

}


# --- Main ---

main() {

    info "Installing Luna — the terminal that remembers"


    local target

    target="$(detect_target)"

    info "Detected platform: ${target}"


    setup_luna_dir


    local version

    version="$(get_latest_version)"


    if [ -n "$version" ]; then

        info "Latest release: v${version}"

        if install_from_release "$version" "$target"; then

            verify_and_next

            exit 0

        fi

        warn "Prebuilt download failed, falling back to building from source..."

    else

        info "No releases found yet, building from source..."

    fi


    install_from_source

    verify_and_next

}


main "$@"
