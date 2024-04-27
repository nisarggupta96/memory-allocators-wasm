APPLICATIONS="crypto"
PACKAGES="buddy dlmalloc rlsf talc wee"

for APPLICATION in ${APPLICATIONS}; do
    echo "Application: ${APPLICATION}"
    cd $APPLICATION
    for PACAKGE in ${PACKAGES}; do
        mkdir -p ../output/crypto && touch ../output/crypto/${PACAKGE}_results.txt
        echo "Package: ${PACAKGE}"
        wasm-pack --log-level error build ${PACAKGE}_${APPLICATION} --release --target web
        cd ${PACAKGE}_${APPLICATION}
        deno run --allow-read bench.js > ../../output/crypto/${PACAKGE}_results.txt
        cd -
    done
    cd -
done
cd ../../
echo "DIR: ${PWD}"

# For image processing
for PACAKGE in ${PACKAGES}; do
    echo "Package: ${PACAKGE}"
    mkdir -p output/image && touch output/image/${PACAKGE}_results.txt
    cd image/${PACAKGE}_image
    cargo bench > ../../output/image/${PACAKGE}_results.txt
    cd ../..
done