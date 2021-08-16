FROM ubuntu:latest

ARG DEBIAN_FRONTEND=noninteractive
# persist at runtime
ENV TZ="America/New York"

RUN apt-get update && apt-get install -y software-properties-common
RUN add-apt-repository -y universe
RUN apt-get update && apt-get -y upgrade
RUN apt-get -y install build-essential git-all wget curl

# Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly

# Shell
RUN apt-get install fish
RUN chsh -s $(which fish)
RUN sh -c "$(curl -fsSL --yes https://starship.rs/install.sh)"

# Neovim
RUN apt-get install neovim
RUN sh -c 'curl -fLo "${XDG_DATA_HOME:-$HOME/.local/share}"/nvim/site/autoload/plug.vim --create-dirs \
    https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim'

ENTRYPOINT [ "fish" ]