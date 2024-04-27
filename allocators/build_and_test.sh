PACKAGES="buddy_alloc dlmalloc rlsf wee_alloc talc"

for PACAKGE in ${PACKAGES}; do
    echo "${PACAKGE}"
    
    wasm-pack --log-level error build ${PACAKGE}_benchmark --release --target web
    
    cd ${PACAKGE}_benchmark
    mkdir -p ../output/${PACAKGE} && touch ../output/${PACAKGE}/random_actions_results.txt && touch ../output/${PACAKGE}/criterion_throughput_results.txt
    deno run --allow-read bench.js > ../output/${PACAKGE}/random_actions_results.txt
    cargo bench > ../output/${PACAKGE}/criterion_throughput_results.txt
    cd -
done