#!/bin/bash

trunk build && \
    { echo '{{ body }}'; cat docs/index.html; } > docs/index.html.tmp && \
    mv docs/index.html.tmp docs/index.html && \
    sed -i '/<body>/a {{ body }}' docs/index.html && \
    cargo run --features hydration > docs/body.html && \
    ltext 'docs/index.html docs/body.html' --raw docs/body.html > docs/index.html.tmp && \
    rm docs/body.html && \
    mv docs/index.html.tmp docs/index.html
