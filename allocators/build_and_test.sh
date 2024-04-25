# PACKAGES="buddy_alloc dlmalloc rlsf wee_alloc talc"
PACKAGES="talc"

for PACAKGE in ${PACKAGES}; do
    echo "${PACAKGE}"
    
    if [ "${PACAKGE}" = "talc" ]; then
        wasm-pack --log-level error build ${PACAKGE}_benchmark --release --target web
    else
        wasm-pack --log-level error build ${PACAKGE}_benchmark --release --target web
    fi
    
    cd ${PACAKGE}_benchmark
    deno run --allow-read bench.js
    cargo bench
    cd -
done