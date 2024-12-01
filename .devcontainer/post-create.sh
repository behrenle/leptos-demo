# install node
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
export NVM_DIR="$([ -z "${XDG_CONFIG_HOME-}" ] && printf %s "${HOME}/.nvm" || printf %s "${XDG_CONFIG_HOME}/nvm")"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh" # This loads nvm
bash -i -c 'nvm install --lts'

# install tailwind
npm i -g tailwindcss
npx playwright install --with-deps

# install rust toolchain and utilities
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install leptosfmt