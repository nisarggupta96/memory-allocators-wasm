# PACKAGES="buddy_alloc dlmalloc rlsf wee_alloc talc"
PACKAGES="wee_alloc"

for PACAKGE in ${PACKAGES}; do
    echo "${PACAKGE}"
    
    if [ "${PACAKGE}" = "talc" ]; then
        wasm-pack --log-level warn build ${PACAKGE}_benchmark --release --target web --features ${PACAKGE}
    else
        wasm-pack --log-level warn build ${PACAKGE}_benchmark --release --target web
    fi
    
    cd ${PACAKGE}_benchmark
    deno run --allow-read bench.js
    cd -
done