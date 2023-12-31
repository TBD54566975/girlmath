# Set the JS_BINDINGS_DIR variable
JS_BINDINGS_DIR := "./bindings/girlmath-js"

default:
    just --list

js:
    just clean_js
    just build_js

# Clean js binding directory
clean_js:
    rm -rf {{JS_BINDINGS_DIR}}/pkg
    rm -rf {{JS_BINDINGS_DIR}}/target
    find {{JS_BINDINGS_DIR}}/dist -type f ! -name 'index.js' -delete

# Build js bindings
build_js:
    cargo build
    {{JS_BINDINGS_DIR}}/scripts/build.sh
    npm install --prefix {{JS_BINDINGS_DIR}}
    node {{JS_BINDINGS_DIR}}/scripts/bundle.js
