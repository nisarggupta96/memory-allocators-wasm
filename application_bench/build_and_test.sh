# APPLICATIONS="crypto image"
APPLICATIONS="crypto"
PACKAGES="buddy dlmalloc rlsf talc wee"

for APPLICATION in ${APPLICATIONS}; do
    echo "Application: ${APPLICATION}"
    cd $APPLICATION
    for PACAKGE in ${PACKAGES}; do
        echo "Package: ${PACAKGE}"
        wasm-pack --log-level error build ${PACAKGE}_${APPLICATION} --release --target web
        cd ${PACAKGE}_${APPLICATION}
        deno run --allow-read bench.js
        cd -
    done
    cd -
done
cd ../../
echo "DIR: ${PWD}"


# For image processing
for PACAKGE in ${PACKAGES}; do
    echo "Package: ${PACAKGE}"
    cd image/${PACAKGE}_image
    cargo bench
    cd ../..
done